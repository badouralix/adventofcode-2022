from tool.runners.python import SubmissionPy


class JonSubmission(SubmissionPy):
  def run(self, s):
    return top_three_elves(s.split("\n"))


# Solution given by ChatGPT, after giving it a hint to fix it.
def top_three_elves(calories):
  # Create a dictionary to store the total calories for each elf
  elf_calories = {}

  # Keep track of the current elf and their total calories
  current_elf = None
  current_calories = 0

  # Iterate through the list of calories
  for calorie in calories:
    # If the current calorie is an empty string, this indicates that we have reached
    # the end of an elf's inventory, so we can add their total calories to the dictionary
    if calorie == "":
      if current_elf:
        elf_calories[current_elf] = current_calories
        current_elf = None
        current_calories = 0
    # Otherwise, this calorie belongs to the current elf, so we can add it to their total
    else:
      # If we haven't encountered this elf before, we can create an entry for them in the dictionary
      if not current_elf:
        current_elf = len(elf_calories) + 1
      current_calories += int(calorie)

  # After iterating through all of the calories, we need to add the last elf's total calories to the dictionary
  if current_elf:
    elf_calories[current_elf] = current_calories

  # Sort the dictionary by the total calories for each elf
  sorted_elf_calories = sorted(elf_calories.items(), key=lambda x: x[1], reverse=True)

  # Get the top three elves and their total calories
  top_elves = sorted_elf_calories[:3]

  # Calculate the total calories carried by the top three elves
  total_calories = sum([x[1] for x in top_elves])

  return total_calories


def test_jon():
  """
  Run `python -m pytest ./day-01/part-2/jon.py` to test the submission.
  """
  assert (
    JonSubmission().run(
      """
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
""".strip()
    )
    == 45000
  )
