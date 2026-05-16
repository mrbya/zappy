#include "test-helpers.hpp"

TEST_F(__ZAPPY_PROJECT_NAME_PASCAL__TestFixture, greting_test) {
    __ZAPPY_PROJECT_NAME_PASCAL__::hello(genRandomString(10).c_str());
}
