#include <chrono>
#include <iostream>
#include <span>
#include <sstream>
#include <string>
#include <unordered_map>
#include <variant>
#include <vector>

inline constexpr int kMaxSizeForUpdate{70000000 - 30000000};

struct FileSystemNode {
  int parent_id;
  std::unordered_map<std::string, int> children;
};

struct Folder {
  std::string name;
};

struct File {
  std::string name;
  int size;
};

using Data = std::variant<Folder, File>;

class FileSystem {
 public:
  FileSystem()
      : nodes_{{-1, {}}}, data_{Folder{"/"}}, position_{0}, total_size_{0} {}

  void MoveUp() { position_ = nodes_.at(position_).parent_id; }

  void MoveTo(const std::string& name) {
    AddFolder(name);
    position_ = nodes_.at(position_).children.at(name);
  }

  void MoveToRoot() { position_ = 0; }

  void AddFile(const std::string& name, int size) {
    if (nodes_.at(position_).children.contains(name)) {
      return;
    }
    nodes_.at(position_).children.insert({name, nodes_.size()});
    nodes_.emplace_back(FileSystemNode{position_, {}});
    data_.emplace_back(File{name, size});
    total_size_ += size;
  }

  void AddFolder(const std::string& name) {
    if (nodes_.at(position_).children.contains(name)) {
      return;
    }
    nodes_.at(position_).children.insert({name, nodes_.size()});
    nodes_.emplace_back(FileSystemNode{position_, {}});
    data_.emplace_back(Folder{name});
  }

  int GetSize() const { return nodes_.size(); }
  int GetTotalSize() const { return total_size_; }

  void Display() const { DisplaySub(0, 0); }

  Data GetData(int position) const { return data_.at(position); }
  FileSystemNode GetNode(int position) const { return nodes_.at(position); }

 private:
  std::vector<FileSystemNode> nodes_;
  std::vector<Data> data_;
  int position_;
  int total_size_;

  void DisplaySub(int position, int offset) const {
    const Data& data = data_.at(position);
    switch (data.index()) {
      case 0: {  // Folder
        std::cerr << std::string(offset, ' ') << std::get<Folder>(data).name
                  << '\n';
        for (const auto& [_, child_id] : nodes_.at(position).children) {
          DisplaySub(child_id, offset + 2);
        }
        break;
      }
      case 1: {  // File
        std::cerr << std::string(offset, ' ') << std::get<File>(data).name
                  << ' ' << std::get<File>(data).size << '\n';
        break;
      }
    }
  }
};

void ParseCd(std::string& line, std::istringstream& iss,
             FileSystem& file_system) {
  std::string target = line.substr(5);
  if (target == "/") {
    file_system.MoveToRoot();
  } else if (target == "..") {
    file_system.MoveUp();
  } else {
    file_system.MoveTo(target);
  }
  std::getline(iss, line);
}

void ParseLs(std::string& line, std::istringstream& iss,
             FileSystem& file_system) {
  for (; std::getline(iss, line);) {
    if (line.at(0) == '$') {
      break;
    }
    if (line.at(0) == 'd') {
      file_system.AddFolder(line.substr(4));
    } else {
      int space_position = line.find(' ');
      int size = std::stoi(line.substr(0, space_position));
      file_system.AddFile(line.substr(space_position + 1), size);
    }
  }
}

void ParseCommand(std::string& line, std::istringstream& iss,
                  FileSystem& file_system) {
  if (line.substr(2, 2) == "cd") {
    ParseCd(line, iss, file_system);
  } else if (line.substr(2, 2) == "ls") {
    ParseLs(line, iss, file_system);
  }
}

int DFSFolderSize(const FileSystem& file_system, int position,
                  int min_folder_size, int& smallest_removable_folder_size) {
  const Data& data = file_system.GetData(position);
  switch (data.index()) {
    case 0: {  // Folder
      int folder_size = 0;
      for (const auto& [_, child_id] : file_system.GetNode(position).children) {
        folder_size += DFSFolderSize(file_system, child_id, min_folder_size,
                                     smallest_removable_folder_size);
      }
      if (folder_size >= min_folder_size &&
          folder_size < smallest_removable_folder_size) {
        smallest_removable_folder_size = folder_size;
      }
      return folder_size;
    }
    case 1:  // File
      return std::get<File>(data).size;
    default:
      exit(1);
  }
}

std::string Run(const std::string& input) {
  // Your code goes here
  std::istringstream iss(input);
  std::string line;
  std::getline(iss, line);
  FileSystem file_system;
  while (!iss.eof()) {
    ParseCommand(line, iss, file_system);
  }
  int smallest_removable_folder_size = 70'000'000;
  int file_system_total_size = file_system.GetTotalSize();
  int min_folder_size = file_system_total_size - kMaxSizeForUpdate;
  DFSFolderSize(file_system, 0, min_folder_size,
                smallest_removable_folder_size);
  return std::to_string(smallest_removable_folder_size);
}

int main(int argc, char* argv[]) {
  if (argc < 2) {
    std::cout << "Missing one argument" << std::endl;
    exit(1);
  }
  auto args = std::span(argv, static_cast<size_t>(argc));

  auto start = std::chrono::high_resolution_clock::now();
  auto answer = Run(args[1]);
  auto end = std::chrono::high_resolution_clock::now();

  std::cout << "_duration:"
            << std::chrono::duration<float, std::milli>(end - start).count()
            << "\n";

  std::cout << answer << "\n";
  return 0;
}
