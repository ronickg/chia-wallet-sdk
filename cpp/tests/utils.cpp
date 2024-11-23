#include <gtest/gtest.h>
#include <vector>
#include <array>
#include <string>
#include "lib.rs.h"
#include <algorithm> // For std::equal
#include <iostream>
class ClvmAllocatorWrapper
{
public:
  ClvmAllocatorWrapper() : allocator(new_clvm()) {}

  rust::Box<Program> alloc(rust::Box<ClvmValue> &&value)
  {
    return allocator->alloc(*value);
  }

  rust::Box<Program> nil()
  {
    return allocator->nil();
  }

private:
  rust::Box<ClvmAllocator> allocator;
};

class ClvmValueWrapper
{
public:
  static rust::Box<ClvmValue> from_int(int64_t value)
  {
    return new_int_value(value);
  }

  static rust::Box<ClvmValue> from_string(const std::string &value)
  {
    return new_string_value(rust::String(value));
  }

  static rust::Box<ClvmValue> from_bool(bool value)
  {
    return new_bool_value(value);
  }

  static rust::Box<ClvmValue> from_bytes(const std::vector<uint8_t> &bytes)
  {
    rust::Vec<uint8_t> vec;
    vec.reserve(bytes.size());
    for (auto b : bytes)
    {
      vec.push_back(b);
    }
    return new_bytes_value(vec);
  }

  static rust::Box<ClvmValue> from_program(rust::Box<Program> &&program)
  {
    return new_program_value(std::move(program));
  }
};

class ClvmTest : public ::testing::Test
{
protected:
  // ClvmTest() : allocator(clvm_new_allocator()) {}

  // rust::Box<ClvmAllocator> allocator;
};

// TEST_F(ClvmTest, TestNil)
// {
//   auto program = allocator->nil();
//   ASSERT_NE(program.into_raw(), nullptr);

//   EXPECT_TRUE(program->is_atom());
//   EXPECT_FALSE(program->is_pair());

//   auto result = program->to_string(*allocator);
//   std::cout << "Program string: " << result << std::endl;
// }
TEST_F(ClvmTest, Hey)
{
  ClvmAllocatorWrapper clvm;

  // Create shared programs
  auto shared1 = clvm.alloc(ClvmValueWrapper::from_int(42));
  auto shared2 = clvm.alloc(ClvmValueWrapper::from_int(42));
}

TEST_F(ClvmTest, TestStringRoundtrip)
{
  auto allocator = new_clvm();
  const std::string expected1 = "hello world";
  auto value1 = new_string_value(expected1);
  auto program1 = allocator->alloc(*value1);

  auto allocator2 = new_clvm();
  const std::string expected2 = "test string";
  auto value2 = new_string_value(expected2);
  auto program2 = allocator2->alloc(*value2);

  auto result1 = program1->to_string(*allocator);
  auto result2 = program2->to_string(*allocator2);

  EXPECT_EQ(result1, expected1);
  EXPECT_EQ(result2, expected2);
}

// Utility function to compare two byte arrays
bool compareBytes(const rust::Vec<uint8_t> &a, const rust::Vec<uint8_t> &b)
{
  return std::equal(a.begin(), a.end(), b.begin());
}

bool compareBytes1(const std::array<uint8_t, 32> &a, const std::array<uint8_t, 32> &b)
{
  return std::equal(a.begin(), a.end(), b.begin());
}

std::string rustStringToStdString(const rust::String &rust_str)
{
  return std::string(rust_str.data(), rust_str.size());
}

void logHex(const std::string &label, const rust::Vec<uint8_t> &vec)
{
  rust::String rust_hex_string = to_hex(vec);                      // Call the exposed Rust function
  std::string hex_string = rustStringToStdString(rust_hex_string); // Convert to std::string

  std::cout << label << ": " << hex_string << std::endl;
}

TEST_F(ClvmTest, AtomRoundtrip)
{
  auto allocator = new_clvm();

  rust::Vec<uint8_t> expected = {1, 2, 3};
  logHex("Expected (hex)", expected);

  auto atom = allocator->alloc(*new_bytes_value(expected));
  auto result = atom->to_atom(*allocator);

  logHex("Result (hex)", result);

  EXPECT_TRUE(compareBytes(result, expected));
}

// TEST_F(ClvmTest, ClvmValueAllocation)
// {
//   auto clvm = new_clvm();
//   auto shared1 = clvm->alloc(*new_int_value(42));
//   auto shared2 = clvm->alloc(*new_int_value(42));

//   auto builder1 = array_builder();
//   builder1->add_int(42)
//       .add_string("Hello, world!")
//       .add_bool(true)
//       .add_bytes(rust::Vec<uint8_t>{1, 2, 3})
//       .add_array(build_from_array(std::move(array_builder()))) // Move the builder
//       .add_int(100)
//       .add_program(std::move(shared1));

//   auto manual = clvm->alloc(*build_from_array(std::move(builder1)));

//   auto builder2 = array_builder();
//   builder2->add_int(42)
//       .add_string("Hello, world!")
//       .add_bool(true)
//       .add_bytes(rust::Vec<uint8_t>{1, 2, 3})
//       .add_array(build_from_array(std::move(array_builder()))) // Move the builder
//       .add_int(100)
//       .add_program(std::move(shared2));

//   auto automatic = clvm->alloc(*build_from_array(std::move(builder2)));

//   auto manual_tree_hash = clvm->tree_hash(*manual);
//   auto auto_tree_hash = clvm->tree_hash(*automatic);
//   EXPECT_TRUE(compareBytes1(manual_tree_hash, auto_tree_hash));
// }
TEST_F(ClvmTest, ClvmValueAllocation)
{
  auto clvm = new_clvm();
  auto shared1 = clvm->alloc(*new_int_value(42));
  auto shared2 = clvm->alloc(*new_int_value(42));

  auto inner_array1 = array_builder();
  inner_array1->add_value(new_int_value(34));
  auto inner_value1 = build_from_array(std::move(inner_array1));

  auto builder1 = array_builder();
  builder1->add_value(new_int_value(42))
      .add_value(new_string_value("Hello, world!"))
      .add_value(new_bool_value(true))
      .add_value(new_bytes_value(rust::Vec<uint8_t>{1, 2, 3}))
      .add_value(std::move(inner_value1))
      .add_value(new_int_value(100))
      .add_value(new_program_value(std::move(shared1)));

  auto manual = clvm->alloc(*build_from_array(std::move(builder1)));

  auto inner_array2 = array_builder();
  inner_array2->add_value(new_int_value(34));
  auto inner_value2 = build_from_array(std::move(inner_array2));

  auto builder2 = array_builder();
  builder2->add_value(new_int_value(42))
      .add_value(new_string_value("Hello, world!"))
      .add_value(new_bool_value(true))
      .add_value(new_bytes_value(rust::Vec<uint8_t>{1, 2, 3}))
      .add_value(std::move(inner_value2))
      .add_value(new_int_value(100))
      .add_value(new_program_value(std::move(shared2)));

  auto automatic = clvm->alloc(*build_from_array(std::move(builder2)));

  auto manual_tree_hash = clvm->tree_hash(*manual);
  auto auto_tree_hash = clvm->tree_hash(*automatic);
  EXPECT_TRUE(compareBytes1(manual_tree_hash, auto_tree_hash));
}
// TEST_F(ClvmTest, ClvmValueAllocation)
// {
//   auto clvm = new_clvm();
//   auto shared1 = clvm->alloc(*new_int_value(42));
//   auto shared2 = clvm->alloc(*new_int_value(42));

//   auto builder1 = array_builder();
//   builder1->add_value(new_int_value(42))
//       .add_value(new_string_value("Hello, world!"))
//       .add_value(new_bool_value(true))
//       .add_value(new_bytes_value(rust::Vec<uint8_t>{1, 2, 3}))
//       .add_value(build_from_array(std::move(array_builder())))
//       .add_value(new_int_value(100))
//       .add_value(new_program_value(std::move(shared1)));

//   auto manual = clvm->alloc(*build_from_array(std::move(builder1)));

//   auto builder2 = array_builder();
//   builder2->add_value(new_int_value(42))
//       .add_value(new_string_value("Hello, world!"))
//       .add_value(new_bool_value(true))
//       .add_value(new_bytes_value(rust::Vec<uint8_t>{1, 2, 3}))
//       .add_value(build_from_array(std::move(array_builder())))
//       .add_value(new_int_value(100))
//       .add_value(new_program_value(std::move(shared2)));

//   auto automatic = clvm->alloc(*build_from_array(std::move(builder2)));

//   auto manual_tree_hash = clvm->tree_hash(*manual);
//   auto auto_tree_hash = clvm->tree_hash(*automatic);
//   EXPECT_TRUE(compareBytes1(manual_tree_hash, auto_tree_hash));
// }
// TEST_F(ClvmTest, ClvmValueAllocation)
// {
//   auto clvm = new_clvm();

//   // Create a shared value (42)
//   auto shared1 = clvm->alloc(*new_int_value(42));
//   auto shared2 = clvm->alloc(*new_int_value(42)); // Create a second instance since we need to move it twice

//   // Create the manual array with all values individually allocated
//   auto manual_array = create_empty_clvm_value_array();
//   append_to_clvm_value_array(*manual_array, new_int_value(42));
//   append_to_clvm_value_array(*manual_array, new_string_value("Hello, world!"));
//   append_to_clvm_value_array(*manual_array, new_bool_value(true));
//   append_to_clvm_value_array(*manual_array, new_bytes_value(rust::Vec<uint8_t>{1, 2, 3}));

//   // Create nested array with [34]
//   auto nested_array = create_empty_clvm_value_array();
//   append_to_clvm_value_array(*nested_array, new_int_value(34));
//   append_to_clvm_value_array(*manual_array, finalize_clvm_value_array(std::move(nested_array)));

//   append_to_clvm_value_array(*manual_array, new_int_value(100));
//   append_to_clvm_value_array(*manual_array, new_program_value(std::move(shared1)));

//   // Finalize the manual array
//   auto manual_clvm_value = finalize_clvm_value_array(std::move(manual_array));
//   auto manual = clvm->alloc(*manual_clvm_value);

//   // Create the automatic array (matching the JS auto version)
//   auto auto_array = create_empty_clvm_value_array();
//   append_to_clvm_value_array(*auto_array, new_int_value(42));
//   append_to_clvm_value_array(*auto_array, new_string_value("Hello, world!"));
//   append_to_clvm_value_array(*auto_array, new_bool_value(true));
//   append_to_clvm_value_array(*auto_array, new_bytes_value(rust::Vec<uint8_t>{1, 2, 3}));

//   // Create nested array with [34]
//   auto auto_nested_array = create_empty_clvm_value_array();
//   append_to_clvm_value_array(*auto_nested_array, new_int_value(34));
//   append_to_clvm_value_array(*auto_array, finalize_clvm_value_array(std::move(auto_nested_array)));

//   append_to_clvm_value_array(*auto_array, new_int_value(100));
//   append_to_clvm_value_array(*auto_array, new_program_value(std::move(shared2)));

//   // Finalize the automatic array
//   auto auto_clvm_value = finalize_clvm_value_array(std::move(auto_array));
//   auto automatic = clvm->alloc(*auto_clvm_value);

//   // Compare tree hashes
//   auto manual_tree_hash = clvm->tree_hash(*manual);
//   auto auto_tree_hash = clvm->tree_hash(*automatic);
//   EXPECT_TRUE(compareBytes1(manual_tree_hash, auto_tree_hash));
// }
// TEST_F(ClvmTest, TestStringRoundtripWrongAllocator)
// {
//   auto allocator = clvm_new_allocator();
//   auto allocator2 = clvm_new_allocator();
//   const std::string expected = "hello world";
//   auto value = new_string_value(expected);
//   auto program = allocator->alloc(*value);

//   EXPECT_THROW({
//     program->to_string(*allocator2); // Using wrong allocator
//   },
//                std::runtime_error);
// }
// class ChiaFFITest : public ::testing::Test
// {
// protected: // Change from private to protected
//   void SetUp() override
//   {
//   }

//   void TearDown() override
//   {
//     // Cleanup, if necessary
//   }

//   bool starts_with(const std::string &str, const std::string &prefix)
//   {
//     return str.size() >= prefix.size() && std::equal(prefix.begin(), prefix.end(), str.begin());
//   }
// };

// TEST_F(ChiaFFITest, FromHexBasic)
// {
//   auto bytes = from_hex("48656c6c6f");
//   ASSERT_EQ(bytes.size(), 5);
//   EXPECT_EQ(bytes[0], 0x48);
//   EXPECT_EQ(bytes[1], 0x65);
//   EXPECT_EQ(bytes[2], 0x6c);
//   EXPECT_EQ(bytes[3], 0x6c);
//   EXPECT_EQ(bytes[4], 0x6f);
// }

// TEST_F(ChiaFFITest, ToHexBasic)
// {
//   std::vector<uint8_t> input = {0x48, 0x65, 0x6c, 0x6c, 0x6f};
//   auto hex = to_hex(rust::Slice<const uint8_t>(input.data(), input.size()));
//   EXPECT_EQ(hex, "48656c6c6f");
// }

// TEST_F(ChiaFFITest, EmptyInput)
// {
//   EXPECT_TRUE(from_hex("").empty());
//   EXPECT_EQ(to_hex(rust::Slice<const uint8_t>(nullptr, 0)), "");
// }

// TEST_F(ChiaFFITest, InvalidHex)
// {
//   EXPECT_TRUE(from_hex("invalid").empty());
//   EXPECT_TRUE(from_hex("123g").empty());
// }

// TEST_F(ChiaFFITest, CBytes32Basic)
// {
//   // Create CBytes32 instances
//   std::array<uint8_t, 32> bytes1{};
//   std::array<uint8_t, 32> bytes2{};
//   bytes1[0] = 0x01;
//   bytes2[0] = 0x02;

//   CBytes32 parent{bytes1};
//   CBytes32 puzzle{bytes2};

//   // Verify bytes are set correctly
//   EXPECT_EQ(parent.bytes[0], 0x01);
//   EXPECT_EQ(puzzle.bytes[0], 0x02);

//   // Test remaining bytes are zero
//   for (size_t i = 1; i < 32; i++)
//   {
//     EXPECT_EQ(parent.bytes[i], 0x00);
//     EXPECT_EQ(puzzle.bytes[i], 0x00);
//   }
// }

// TEST_F(ChiaFFITest, CoinCreation)
// {
//   // Setup test data
//   std::array<uint8_t, 32> parent_bytes{};
//   std::array<uint8_t, 32> puzzle_bytes{};
//   parent_bytes[0] = 0x01;
//   puzzle_bytes[0] = 0x02;

//   CBytes32 parent{parent_bytes};
//   CBytes32 puzzle{puzzle_bytes};
//   uint64_t amount = 1000;

//   // Create coin
//   Coin coin = new_coin(parent, puzzle, amount);

//   // Verify coin properties
//   EXPECT_EQ(coin.parent_coin_info.bytes[0], 0x01);
//   EXPECT_EQ(coin.puzzle_hash.bytes[0], 0x02);
//   EXPECT_EQ(coin.amount, 1000);

//   // Verify remaining bytes
//   for (size_t i = 1; i < 32; i++)
//   {
//     EXPECT_EQ(coin.parent_coin_info.bytes[i], 0x00);
//     EXPECT_EQ(coin.puzzle_hash.bytes[i], 0x00);
//   }
// }

// TEST_F(ChiaFFITest, CoinWithFullBytes32)
// {
//   // Create fully populated byte arrays
//   std::array<uint8_t, 32> parent_bytes;
//   std::array<uint8_t, 32> puzzle_bytes;

//   for (size_t i = 0; i < 32; i++)
//   {
//     parent_bytes[i] = i;
//     puzzle_bytes[i] = 32 - i;
//   }

//   CBytes32 parent{parent_bytes};
//   CBytes32 puzzle{puzzle_bytes};
//   uint64_t amount = UINT64_MAX;

//   Coin coin = new_coin(parent, puzzle, amount);

//   // Verify all bytes
//   for (size_t i = 0; i < 32; i++)
//   {
//     EXPECT_EQ(coin.parent_coin_info.bytes[i], i);
//     EXPECT_EQ(coin.puzzle_hash.bytes[i], 32 - i);
//   }
//   EXPECT_EQ(coin.amount, UINT64_MAX);
// }

// TEST_F(ChiaFFITest, CoinId)
// {
//   // Create test coin
//   std::array<uint8_t, 32> parent_bytes{};
//   std::array<uint8_t, 32> puzzle_bytes{};
//   for (size_t i = 0; i < 32; i++)
//   {
//     parent_bytes[i] = i;
//     puzzle_bytes[i] = 32 - i;
//   }

//   CBytes32 parent{parent_bytes};
//   CBytes32 puzzle{puzzle_bytes};
//   uint64_t amount = 1000;

//   Coin coin = new_coin(parent, puzzle, amount);
//   CBytes32 coin_id = get_coin_id(coin); // Removed &

//   // Verify coin ID is non-zero and consistent
//   bool all_zero = true;
//   for (size_t i = 0; i < 32; i++)
//   {
//     if (coin_id.bytes[i] != 0)
//     {
//       all_zero = false;
//       break;
//     }
//   }
//   EXPECT_FALSE(all_zero);

//   // Verify same input produces same coin ID
//   CBytes32 coin_id2 = get_coin_id(coin); // Removed &
//   for (size_t i = 0; i < 32; i++)
//   {
//     EXPECT_EQ(coin_id.bytes[i], coin_id2.bytes[i]);
//   }
// }