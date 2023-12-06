import sys
import pandas as pd
import matplotlib.pyplot as plt

def generate_bar_graph(file_path):
    data = pd.read_csv(file_path)

    
    data['Execution Time (s)'] = pd.to_numeric(data['Execution Time (s)'], errors='coerce')

    grouped_data = data.groupby(['Script', 'Language'])['Execution Time (s)'].mean().unstack()

    num_iterations = data['Iteration'].max()

    print("Calculated Averages:")
    print(grouped_data)

    ax = grouped_data.plot(kind='barh', figsize=(10, 6), colormap='viridis')

    plt.title(f'Average Execution Time for Scripts in Different Languages\n(Iterations: {num_iterations})')
    plt.xlabel('Average Execution Time (s)')
    plt.ylabel('Scripts')
    plt.legend(title='Language')
    plt.grid(axis='x')
    plt.tight_layout()

    for rect in ax.patches:
        width = rect.get_width()
        plt.text(width, rect.get_y() + rect.get_height() / 2, f'{width:.2f}', ha='left', va='center')

   
    for i, row in data.iterrows():
        if pd.isnull(row['Execution Time (s)']):
            ax.text(0, i, 'DNF', ha='left', va='center', color='red')

    plt.savefig('output_plot.jpg', format='jpg')
    plt.show()

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <file_path>")
    else:
        file_path = sys.argv[1]
        generate_bar_graph(file_path)
