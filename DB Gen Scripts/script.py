from sklearn import datasets
from sklearn.model_selection import cross_val_score
from sklearn.linear_model import LinearRegression
from sklearn.cluster import KMeans
from sklearn.ensemble import RandomForestClassifier, GradientBoostingClassifier
from sklearn.svm import SVC
from time import process_time
# Load the iris dataset
iris = datasets.load_iris()
X = iris.data
y = iris.target

# Define the models
models = [
    ('LR', LinearRegression()),
    ('RFC', RandomForestClassifier()),
    ('GBC', GradientBoostingClassifier()),
    ('KM', KMeans(n_init=10))
]

# Compare models
for name, model in models:
    start = process_time()
    scores = cross_val_score(model, X, y, cv=5, scoring='accuracy')
    print(f'{name}(Accuracy): {scores.mean():.2f} (+/- {scores.std() * 2:.2f})')
    end = process_time()
    print(f'{name}(Speed): {end - start}s')