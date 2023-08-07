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

void print_message(bool b)
{
    std::cout << (b ? "Yes" : "No") << std::endl;
}

int main()
{
    int n;
    std::string line;
    std::cin >> n;
    for (int i = 0; i < n; i++)
    {
        std::cin >> line;
        print_message(check_str(line));
    }
}
