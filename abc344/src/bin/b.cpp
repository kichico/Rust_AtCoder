#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
using namespace std;
int main(){
    string s;
    vector<string> ss;
    while(!cin.eof()){ 
        string in;
        cin>>in;
        ss.push_back(in);
        if (in=="0"){
            break;
        }
        }
    reverse(ss.begin(),ss.end());
    for(auto c:ss){
        cout<<c<<endl;
    }
    return 0;
}