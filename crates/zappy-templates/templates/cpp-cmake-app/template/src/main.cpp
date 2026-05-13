#include <main.hpp>

int main (int argc, char *argv[]) {

    while(true) {
        std::cout << "Hello from __ZAPPY_PROJECT_NAME__!" << std::endl;
        std::this_thread::sleep_for(std::chrono::seconds(1));
    }

    return 0;
}
