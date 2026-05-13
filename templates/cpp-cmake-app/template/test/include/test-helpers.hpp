#ifndef TEST_HELPERS_HPP
#define TEST_HELPERS_HPP

#include <chrono>
#include <cstdint>
#include <memory>
#include <random>
#include <string>

#include <gtest/gtest.h>
#include <gmock/gmock.h>

class __ZAPPY_PROJECT_NAME_PASCAL__TestFixture: public::testing::Test {
protected:
    std::random_device rd;
    std::mt19937_64 gen;

public:
    uint32_t genRandomU32(uint32_t min = 0, uint32_t max = 0xffffffff) {
        std::uniform_int_distribution<uint32_t> distrib(min, max);
        return distrib(gen);
    }

protected:
    void SetUp() override {
        gen = std::mt19937_64(rd());
    }
};

#endif
