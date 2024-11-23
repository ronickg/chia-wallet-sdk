#include <gtest/gtest.h>
#include <vector>
#include <array>
#include <string>
#include <algorithm> // For std::equal
#include <iostream>
// #include "chia_wallet_ffi.h"
#include "clvm.hpp"
#include "program.hpp"
#include "bytes.hpp"
#include "clvm_value.hpp"

using namespace chia;

class ClvmTest : public ::testing::Test
{
protected:
  void SetUp() override {}
  void TearDown() override {}
};

// Test Program move operations
TEST_F(ClvmTest, StringRoundTrip)
{
  Clvm clvm;
  auto expected = "Hello, world!";
  auto atom = clvm.allocate(chia::ClvmValue::createString(expected));
  auto result = atom.to_string();
  EXPECT_EQ(result, expected);
}

TEST_F(ClvmTest, ArrayTest)
{
  Clvm clvm;

  // Create test values
  std::vector<chia::ClvmValue> values;
  values.push_back(chia::ClvmValue::createNumber(42));
  values.push_back(chia::ClvmValue::createString("hello"));

  // // Add a bigint (420 in big-endian)
  // std::vector<uint8_t> bigint_data = {0x01, 0xA4};
  // values.push_back(chia::ClvmValue::createBigInt(bigint_data));

  // Create array
  auto array_value = chia::ClvmValue::createArray(values);
  ASSERT_NE(array_value.raw_handle(), nullptr);
  ASSERT_EQ(array_value.raw_handle()->value_type, ClvmValueType::Array);

  // Allocate in CLVM
  auto program = clvm.allocate(array_value);

  // Get the result back as bytes for verification
  // auto result = program.to_bigint_bytes();

  // You would add specific checks here based on your CLVM encoding
  // For example, verifying the encoded bytes match your expected format
  // ASSERT_FALSE(result.empty());
}

TEST_F(ClvmTest, ListRoundTrip)
{
  Clvm clvm;

  // Create array of numbers 0-9
  std::vector<chia::ClvmValue> values;
  for (int i = 0; i < 10; i++)
  {
    values.push_back(chia::ClvmValue::createNumber(i));
  }

  // Create array and allocate
  auto array_value = chia::ClvmValue::createArray(values);
  auto program = clvm.allocate(array_value);

  // Get the result back as a list of items
  auto result = program.to_list();

  // Verify each number matches
  ASSERT_EQ(result.size(), 10);
  for (int i = 0; i < 10; i++)
  {
    EXPECT_EQ(result[i].to_number(), i);
  }
}

TEST_F(ClvmTest, BigIntRoundTrip)
{
  Clvm clvm;
  // Test vector representing big number 12345
  std::vector<uint8_t> bigNum = {0x30}; // Just the value 48
  auto program = clvm.allocate(chia::ClvmValue::createBigInt(bigNum));
  auto result = program.to_bigint_bytes();

  EXPECT_EQ(result.size(), bigNum.size());
}

TEST_F(ClvmTest, BigIntRoundTrip1)
{
  Clvm clvm;
  // 420 in big-endian bytes
  // Test just one simple number first
  std::vector<uint8_t> input = {0xA4}; // 164 in one byte

  // Debug output for initial bytes
  std::cout << "Input bytes: ";
  for (auto b : input)
    std::cout << std::hex << (int)b << " ";
  std::cout << std::endl;

  // Create value and get raw handle for debugging
  auto value = chia::ClvmValue::createBigInt(input);
  auto *value_handle = value.raw_handle();

  // Verify the value type is correct
  std::cout << "Value type: " << (int)value_handle->value_type << std::endl;

  // Get bytes handle pointer for debugging
  auto *bytes_handle = value_handle->data.bigint;
  if (bytes_handle)
  {
    auto len = bytes_len(bytes_handle);
    std::cout << "Bytes handle len: " << len << std::endl;

    std::vector<uint8_t> intermediate(len);
    if (bytes_copy_to(bytes_handle, intermediate.data(), len))
    {
      std::cout << "Intermediate bytes: ";
      for (auto b : intermediate)
        std::cout << std::hex << (int)b << " ";
      std::cout << std::endl;
    }
  }

  // Allocate and get result
  auto result = clvm.allocate(value).to_bigint_bytes();

  std::cout << "Result bytes: ";
  for (auto b : result)
    std::cout << std::hex << (int)b << " ";
  std::cout << std::dec << std::endl;

  EXPECT_EQ(result.size(), input.size()) << "Size mismatch";
  EXPECT_TRUE(std::equal(result.begin(), result.end(), input.begin())) << "Byte mismatch";
}

// Test basic construction and handle validity
TEST_F(ClvmTest, Construction)
{
  EXPECT_NO_THROW({
    Clvm clvm;
    EXPECT_NE(clvm.raw_handle(), nullptr);
  });
}

TEST_F(ClvmTest, CreateEmpty)
{
  Bytes bytes;
  EXPECT_TRUE(bytes.empty());
  EXPECT_EQ(bytes.size(), 0);
}

TEST_F(ClvmTest, CreateFromVector)
{
  std::vector<uint8_t> data = {1, 2, 3, 4, 5};
  Bytes bytes(data);

  EXPECT_FALSE(bytes.empty());
  EXPECT_EQ(bytes.size(), 5);

  auto copy = bytes.to_vector();
  EXPECT_EQ(copy.size(), 5);
  EXPECT_EQ(copy[0], 1);
  EXPECT_EQ(copy[4], 5);
}

TEST_F(ClvmTest, ToVector)
{
  std::vector<uint8_t> original = {1, 2, 3, 4, 5};
  Bytes bytes(original);

  auto copy = bytes.to_vector();
  EXPECT_EQ(copy.size(), original.size());
  EXPECT_EQ(copy, original);
}

TEST_F(ClvmTest, MoveConstructionBytes)
{
  std::vector<uint8_t> data = {1, 2, 3};
  Bytes original(data);
  Bytes moved(std::move(original));

  // Original should be empty
  EXPECT_EQ(original.raw_handle(), nullptr);

  // Moved should have the data
  EXPECT_EQ(moved.size(), 3);
  auto copy = moved.to_vector();
  EXPECT_EQ(copy[0], 1);
  EXPECT_EQ(copy[2], 3);
}

TEST_F(ClvmTest, MoveAssignmentBytes)
{
  std::vector<uint8_t> data1 = {1, 2, 3};
  std::vector<uint8_t> data2 = {4, 5, 6};

  Bytes bytes1(data1);
  Bytes bytes2(data2);

  bytes1 = std::move(bytes2);

  // bytes2 should be empty
  EXPECT_EQ(bytes2.raw_handle(), nullptr);

  // bytes1 should have data2's content
  auto copy = bytes1.to_vector();
  EXPECT_EQ(copy.size(), 3);
  EXPECT_EQ(copy[0], 4);
  EXPECT_EQ(copy[2], 6);
}

TEST_F(ClvmTest, EmptyVector)
{
  std::vector<uint8_t> empty;
  Bytes bytes(empty);

  EXPECT_TRUE(bytes.empty());
  EXPECT_EQ(bytes.size(), 0);

  auto copy = bytes.to_vector();
  EXPECT_TRUE(copy.empty());
}

TEST_F(ClvmTest, LargeData)
{
  std::vector<uint8_t> large_data(1000);
  // Fill with incrementing values
  for (size_t i = 0; i < large_data.size(); ++i)
  {
    large_data[i] = static_cast<uint8_t>(i % 256);
  }

  Bytes bytes(large_data);
  EXPECT_EQ(bytes.size(), 1000);

  auto copy = bytes.to_vector();
  EXPECT_EQ(copy, large_data);
}

TEST_F(ClvmTest, MultipleToVector)
{
  std::vector<uint8_t> data = {1, 2, 3};
  Bytes bytes(data);

  auto copy1 = bytes.to_vector();
  auto copy2 = bytes.to_vector();

  EXPECT_EQ(copy1, copy2);
  EXPECT_EQ(copy1, data);
}

TEST_F(ClvmTest, EmptyConstruction)
{
  EXPECT_NO_THROW({
    chia::Bytes bytes;
  });
}

TEST_F(ClvmTest, DestructorSafety)
{
  std::vector<uint8_t> data = {1, 2, 3};

  // Test multiple creation/destruction cycles
  for (int i = 0; i < 100; ++i)
  {
    Bytes bytes(data);
    EXPECT_EQ(bytes.size(), 3);
  }
}

// Test cleanup callback
TEST_F(ClvmTest, CleanupCallback)
{
  bool cleanup_called = false;
  {
    Clvm clvm;
    clvm.setCleanupCallback([&cleanup_called]()
                            {
            cleanup_called = true;
            std::cout << "Cleaning up CLVM\n"; });
  } // clvm goes out of scope here
  EXPECT_TRUE(cleanup_called);
}

// Test move construction
TEST_F(ClvmTest, MoveConstruction)
{
  bool cleanup_called = false;
  Clvm clvm1;
  auto original_handle = clvm1.raw_handle();
  clvm1.setCleanupCallback([&cleanup_called]()
                           { cleanup_called = true; });

  Clvm clvm2(std::move(clvm1));
  EXPECT_EQ(clvm2.raw_handle(), original_handle);
  // After move, cleanup shouldn't be called yet
  EXPECT_FALSE(cleanup_called);
}

// Test move assignment
TEST_F(ClvmTest, MoveAssignment)
{
  bool cleanup1_called = false;
  bool cleanup2_called = false;

  Clvm clvm1;
  auto original_handle = clvm1.raw_handle();
  clvm1.setCleanupCallback([&cleanup1_called]()
                           { cleanup1_called = true; });

  {
    Clvm clvm2;
    clvm2.setCleanupCallback([&cleanup2_called]()
                             { cleanup2_called = true; });

    clvm2 = std::move(clvm1);
    EXPECT_EQ(clvm2.raw_handle(), original_handle);
    // cleanup2 should be called when clvm2's original impl is destroyed
    EXPECT_TRUE(cleanup2_called);
    // cleanup1 shouldn't be called yet
    EXPECT_FALSE(cleanup1_called);
  }
  // Now cleanup1 should be called because clvm2 went out of scope
  EXPECT_TRUE(cleanup1_called);
}

// Test multiple moves
TEST_F(ClvmTest, ChainedMoves)
{
  bool cleanup_called = false;
  Clvm clvm1;
  auto original_handle = clvm1.raw_handle();
  clvm1.setCleanupCallback([&cleanup_called]()
                           { cleanup_called = true; });

  Clvm clvm2(std::move(clvm1));
  Clvm clvm3(std::move(clvm2));
  EXPECT_EQ(clvm3.raw_handle(), original_handle);
  EXPECT_FALSE(cleanup_called);
}

// Test cleanup callback modification
TEST_F(ClvmTest, ModifyCleanupCallback)
{
  int cleanup_count = 0;
  {
    Clvm clvm;
    clvm.setCleanupCallback([&cleanup_count]()
                            { cleanup_count++; });

    // Modify callback
    clvm.setCleanupCallback([&cleanup_count]()
                            { cleanup_count += 2; });
  }
  // Should call the last set callback
  EXPECT_EQ(cleanup_count, 2);
}

// Test multiple instances
TEST_F(ClvmTest, MultipleInstances)
{
  std::vector<bool> cleanups(3, false);
  {
    Clvm clvm1;
    Clvm clvm2;
    Clvm clvm3;

    clvm1.setCleanupCallback([&cleanups]()
                             { cleanups[0] = true; });
    clvm2.setCleanupCallback([&cleanups]()
                             { cleanups[1] = true; });
    clvm3.setCleanupCallback([&cleanups]()
                             { cleanups[2] = true; });

    EXPECT_NE(clvm1.raw_handle(), clvm2.raw_handle());
    EXPECT_NE(clvm2.raw_handle(), clvm3.raw_handle());
    EXPECT_NE(clvm1.raw_handle(), clvm3.raw_handle());
  }
  // All cleanups should have been called
  EXPECT_TRUE(cleanups[0]);
  EXPECT_TRUE(cleanups[1]);
  EXPECT_TRUE(cleanups[2]);
}
// Test Program creation and basic operations
TEST_F(ClvmTest, ProgramCreationNil)
{
  Clvm clvm;
  EXPECT_NO_THROW({
    auto program = Program::nil(clvm);
    EXPECT_TRUE(program.is_atom());
    EXPECT_FALSE(program.is_pair());
  });
}

// Test Program move operations
TEST_F(ClvmTest, ProgramMove)
{
  Clvm clvm;
  auto program1 = Program::nil(clvm);

  // Test move construction
  Program program2(std::move(program1));
  EXPECT_TRUE(program2.is_atom());

  // Test move assignment
  Program program3 = Program::nil(clvm);
  program3 = std::move(program2);
  EXPECT_TRUE(program3.is_atom());
}

// Test Program tree hash
TEST_F(ClvmTest, ProgramTreeHash)
{
  Clvm clvm;
  auto program = Program::nil(clvm);

  auto hash = program.tree_hash();

  // Convert to hex and verify format
  std::string hex = Program::hash_to_hex(hash);
  EXPECT_EQ(hex.length(), 64); // 32 bytes = 64 hex chars
}

// Test Program lifecycle with CLVM
TEST_F(ClvmTest, ProgramWithClvmLifecycle)
{
  Program *program_ptr = nullptr;
  {
    Clvm clvm;
    auto program = Program::nil(clvm);
    EXPECT_TRUE(program.is_atom());

    // Create another program while first still exists
    auto program2 = Program::nil(clvm);
    EXPECT_TRUE(program2.is_atom());
  } // Both programs and CLVM should be cleaned up here
}

// Test multiple programs with same CLVM
TEST_F(ClvmTest, MultipleProgramsSameClvm)
{
  Clvm clvm;
  std::vector<Program> programs;

  // Create multiple programs
  for (int i = 0; i < 5; i++)
  {
    programs.push_back(Program::nil(clvm));
  }

  // Verify all programs are valid
  for (const auto &program : programs)
  {
    EXPECT_TRUE(program.is_atom());
    auto hash = program.tree_hash();
  }
}

// Test Program hash consistency
TEST_F(ClvmTest, ProgramHashConsistency)
{
  Clvm clvm;
  auto program1 = Program::nil(clvm);
  auto program2 = Program::nil(clvm);

  auto hash1 = program1.tree_hash();
  auto hash2 = program2.tree_hash();

  // NIL programs should have the same hash
  EXPECT_EQ(hash1, hash2);
}

// Test Program cleanup with CLVM cleanup
TEST_F(ClvmTest, ProgramCleanupWithClvm)
{
  bool clvm_cleanup_called = false;
  {
    Clvm clvm;
    clvm.setCleanupCallback([&clvm_cleanup_called]()
                            { clvm_cleanup_called = true; });

    // Create and destroy programs before CLVM cleanup
    {
      auto program1 = Program::nil(clvm);
      auto program2 = Program::nil(clvm);
      EXPECT_FALSE(clvm_cleanup_called);
    } // Programs cleaned up here

    EXPECT_FALSE(clvm_cleanup_called);
  } // CLVM cleaned up here
  EXPECT_TRUE(clvm_cleanup_called);
}

// Test invalid operations with moved Program
TEST_F(ClvmTest, ProgramInvalidAfterMove)
{
  Clvm clvm;
  Program program1 = Program::nil(clvm);
  Program program2(std::move(program1));

  // program1 should be in moved-from state
  auto hash = program1.tree_hash();

  // program2 should be valid
  hash = program2.tree_hash();
  // EXPECT_TRUE(hash);
}