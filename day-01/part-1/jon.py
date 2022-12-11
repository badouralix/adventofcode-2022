from tool.runners.python import SubmissionPy


class JonSubmission(SubmissionPy):
    def run(self, s):
        # This is a solution produced by ChatGPT, after giving it a few hints.

        # Read the input from the user
        calories = s # input()

        # Split the input into lines
        calories = calories.split('\n')

        # Keep track of the Elf with the most Calories
        max_calories = 0
        max_elf = 0

        # Initialize the total Calories for the current Elf to 0
        total_calories = 0

        # Loop through the input
        for line in calories:
            # If the line is empty, it marks the end of an Elf's inventory
            if line == '':
                # Check if the current Elf has more Calories than the Elf with the most Calories
                if total_calories > max_calories:
                    # If so, update the Elf with the most Calories
                    max_calories = total_calories
                    max_elf += 1
                # Reset the total Calories for the next Elf
                total_calories = 0
            else:
                # Add the Calories for the current item to the total for the current Elf
                total_calories += int(line)

        # Print the Elf with the most Calories and their total Calories
        #print(f'Elf {max_elf} is carrying the most Calories: {max_calories}')

        return max_calories


def test_jon():
    """
    Run `python -m pytest ./day-01/part-1/jon.py` to test the submission.
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
        == 24000
    )
