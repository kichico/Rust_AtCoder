#include <string>
#include <iostream>
#include <set>
using namespace std;
using ll = long long;
int main(void)
{
    ll N;
    cin >> N;
    string s;
    cin >> s;
    set<string> ans;
    string ss = s.substr(0, 1);
    ans.emplace(ss);
    for (int i = 0; i < N; ++i)
    {
        if (s[i] != s[i - 1])
        {
            ss = s[i];
            ans.emplace(ss);
        }
        else
        {
            ss += s[i];
            ans.emplace(ss);
        }
    }
    cout << ans.size() << endl;
    return 0;
}