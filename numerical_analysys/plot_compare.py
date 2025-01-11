import pandas as pd
from numpy import sin, cos, log, exp, sinh
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

        for i, x in enumerate(data['x']):
            std_data.append(
                abs(data['y'][i]-sin(x)*exp(x))
            )

        # Plot the data
        plt.figure(figsize=(10, 6))
        # plt.plot(data['x'], data['y'], marker='3', linestyle='-', color='r')
        plt.plot(data['x'], std_data, marker='none', linestyle='-', color='g')
        plt.legend(["Absolute difference", "glibc"])
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
