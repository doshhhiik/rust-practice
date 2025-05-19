#include <iostream>
#include <vector>
#include <cmath>

using namespace std;

int diagonalDifference(vector<vector<int>> arr) {
    int n = arr.size();
    int primarySum = 0, secondarySum = 0;

    for (int i = 0; i < n; i++) {
        primarySum += arr[i][i];            
       secondarySum += arr[i][n - i - 1];  
    }

    return abs(primarySum - secondarySum);  
}

int main() {
    int n;
    cin >> n;
    vector<vector<int>> arr(n, vector<int>(n));

    for (int i = 0; i < n; i++)
        for
