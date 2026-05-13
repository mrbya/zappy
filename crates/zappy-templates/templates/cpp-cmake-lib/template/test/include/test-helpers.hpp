#ifndef TEST_HELPERS_HPP
#define TEST_HELPERS_HPP

#include <chrono>
#include <cstdint>
#include <memory>
#include <random>
#include <string>

#include <__ZAPPY_PROJECT_NAME_SNAKE__.hpp>

#include <gtest/gtest.h>
#include <gmock/gmock.h>

using namespace __ZAPPY_PROJECT_NAME_SNAKE__;

class __ZAPPY_PROJECT_NAME_PASCAL__TestFixture: public::testing::Test {
protected:
    std::random_device rd;
    std::mt19937_64 gen;

public:
    uint32_t genRandomU32(uint32_t min = 0, uint32_t max = 0xffffffff) {
        std::uniform_int_distribution<uint32_t> distrib(min, max);
        return distrib(gen);
    }

    std::string generate_random_string(size_t len) {
        const std::string characters = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        std::uniform_int_distribution<> distrib(0, characters.size() - 1);

        std::string random_string;
        for (size_t i = 0; i < len; ++i) {
            random_string += characters[distrib(gen)];
        }

        return random_string;
    }

protected:
    void SetUp() override {
        gen = std::mt19937_64(rd());
    }
};

#endif
