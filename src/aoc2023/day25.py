
import numpy as np
from numpy.linalg import eig

nodes = {}
index = 0
indices = {}
with open('./inputs/aoc2023/input_day25.txt', 'r') as file:
    for i, line in enumerate(file):
        u,vs = line.split(": ")
        if u not in indices:
                indices[u] = index
                index+=1
        for v in vs.split():
            if v not in indices:
                indices[v] = index
                index+=1
            nodes.setdefault(u, []).append(v)
            nodes.setdefault(v, []).append(u)
            
matrix = np.zeros((len(indices),len(indices)))
for i,vs in enumerate(nodes.values()):
    matrix[i,i]=len(vs)
    for v in vs:
        matrix[i,indices[v]] = -1
        
w,v=eig(matrix) 
n = np.sum(v[1] >= 0) #Fiedler vector
print(n*(len(indices)-n))