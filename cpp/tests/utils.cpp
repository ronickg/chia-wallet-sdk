#include <gtest/gtest.h>
#include <vector>
#include <array>
#include <string>
#include "lib.rs.h"
#include <algorithm> // For std::equal

class ClvmTest : public ::testing::Test
{
protected:
  ClvmTest() : allocator(create_allocator()) {}

  rust::Box<ClvmAllocator> allocator;
};

TEST_F(ClvmTest, CreateAllocator)
{
  auto alloc = create_allocator();
  // Check if operator-> works (means not null)
  ASSERT_NE(&(*alloc), nullptr);
}

TEST_F(ClvmTest, CreateNilProgram)
{
  auto program = nil(*allocator);
  ASSERT_NE(&(*program), nullptr);
  EXPECT_TRUE(program->is_atom());
  EXPECT_FALSE(program->is_pair());
}

TEST_F(ClvmTest, SerializeNilProgram)
{
  auto program = nil(*allocator);
  auto result = program->serialize();

  rust::Vec<uint8_t> expected = {0x80};

  ASSERT_EQ(result.size(), expected.size());
  for (size_t i = 0; i < result.size(); i++)
  {
    EXPECT_EQ(result[i], expected[i]);
  }
}

TEST_F(ClvmTest, DeserializeNilProgram)
{
  rust::Vec<uint8_t> nil_bytes = {0x80};
  rust::Slice<const uint8_t> slice(nil_bytes.data(), nil_bytes.size());

  auto program = deserialize(*allocator, slice);
  ASSERT_NE(&(*program), nullptr);
  EXPECT_TRUE(program->is_atom());
  EXPECT_FALSE(program->is_pair());
}

TEST_F(ClvmTest, SerializeDeserializeRoundTrip)
{
  auto original = nil(*allocator);
  ASSERT_NE(&(*original), nullptr);

  auto serialized = original->serialize();
  ASSERT_FALSE(serialized.empty());

  auto deserialized = deserialize(*allocator, rust::Slice<const uint8_t>(serialized.data(), serialized.size()));
  ASSERT_NE(&(*deserialized), nullptr);

  EXPECT_EQ(original->is_atom(), deserialized->is_atom());
  EXPECT_EQ(original->is_pair(), deserialized->is_pair());

  auto reserialized = deserialized->serialize();
  ASSERT_EQ(serialized.size(), reserialized.size());
  for (size_t i = 0; i < serialized.size(); i++)
  {
    EXPECT_EQ(serialized[i], reserialized[i]);
  }
}

TEST_F(ClvmTest, DeserializeInvalidBytes)
{
  rust::Vec<uint8_t> invalid_bytes = {0xFF, 0xFF, 0xFF};
  rust::Slice<const uint8_t> slice(invalid_bytes.data(), invalid_bytes.size());

  EXPECT_THROW({ deserialize(*allocator, slice); }, std::exception); // Change from rust::Error to std::exception
}

TEST_F(ClvmTest, HandleEmptyInput)
{
  rust::Vec<uint8_t> empty_bytes;
  rust::Slice<const uint8_t> slice(empty_bytes.data(), empty_bytes.size());

  EXPECT_THROW({ deserialize(*allocator, slice); }, std::exception); // Change from rust::Error to std::exception
}

TEST_F(ClvmTest, StringRoundtrip)
{
  std::string expected = "hello world";
  auto value = from_string(expected);
  auto result = allocate_value(*value, *allocator);

  ASSERT_TRUE(result.success);
  auto str = result.node_ptr_value->to_string();
  ASSERT_EQ(str, expected);
}

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