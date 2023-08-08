#include <iostream>
#include <string>
#include <stack>

bool check_str(std::string str)
{
    std::stack<char> st;
    if (str.size() == 0)
        return true;
    for (auto it = str.begin(); it != str.end(); it++)
    {
        char c = *it;
        if (c == '(' || c == '[')
        {
            st.push(c);
            continue;
        }
        char expectedOpeningTag = c == ')' ? '(' : '[';
        if (st.size() == 0 || st.top() != expectedOpeningTag)
            return false;
        st.pop();
    }
    return st.size() == 0;
}

int main()
{
    int n;
    std::string line;

    std::cin >> n;
    std::cin.get();

    for (int i = 0; i < n; i++)
    {
        std::getline(std::cin, line, '\n');
        std::cout << (check_str(line) ? "Yes" : "No") << std::endl;
    }
}
