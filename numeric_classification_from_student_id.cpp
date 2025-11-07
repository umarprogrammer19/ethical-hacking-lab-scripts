#include <iostream>
#include <string>
using namespace std;

bool isPrime(int n) {
    if (n < 2) return false;
    for (int i = 2; i * i <= n; i++)
        if (n % i == 0) return false;
    return true;
}

int main() {
    string id = "BC240222474";
    string name = "Asad Tariq";
    cout << id << " belongs to " << name << endl;

    string numbers = "";
    for (char c : id) {
        if (isdigit(c)) numbers += c;
    }

    int zeroCount = 0, evenCount = 0, oddCount = 0, primeCount = 0;

    for (char c : numbers) {
        int digit = c - '0';
        if (digit == 0) {
            cout << digit << " zero found in id" << endl;
            zeroCount++;
        } 
        else if (isPrime(digit)) {
            cout << digit << " is a prime number" << endl;
            primeCount++;
        } 
        else if (digit % 2 == 0) {
            cout << digit << " is an even number" << endl;
            evenCount++;
        } 
        else {
            cout << digit << " is an odd number" << endl;
            oddCount++;
        }
    }

    cout << "Total Counts:\n";
    cout << "Zeros: " << zeroCount << endl;
    cout << "Even numbers: " << evenCount << endl;
    cout << "Odd numbers: " << oddCount << endl;
    cout << "Prime numbers: " << primeCount << endl;

    return 0;
}
