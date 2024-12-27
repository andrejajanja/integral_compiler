import pandas as pd
from numpy import sin, cos, log, exp
import matplotlib
matplotlib.use('Qt5Agg')
import matplotlib.pyplot as plt

# Read CSV and plot values
def plot_csv_values(csv_file):
    try:
        # Read the CSV file
        data = pd.read_csv(csv_file)
        
        # Check if required columns exist
        if 'x' not in data.columns or 'y' not in data.columns:
            raise ValueError("CSV must have 'x' and 'y' columns.")
        
        std_data = []

        for x in data['x']:
            std_data.append(
                log(x)*cos(x)
            )

        # Plot the data
        plt.figure(figsize=(10, 6))
        plt.plot(data['x'], data['y'], marker='o', linestyle='-', color='r')
        plt.plot(data['x'], std_data, marker='o', linestyle='-', color='g')
        plt.title('Function values')
        plt.xlabel('x')
        plt.ylabel('y')
        plt.grid(True)
        plt.show()
    
    except FileNotFoundError:
        print(f"Error: The file '{csv_file}' was not found.")
    except pd.errors.EmptyDataError:
        print("Error: The file is empty.")
    except ValueError as ve:
        print(f"Error: {ve}")
    except Exception as e:
        print(f"An error occurred: {e}")

csv_file = "./benchmarks/poly_func.csv"
plot_csv_values(csv_file)
