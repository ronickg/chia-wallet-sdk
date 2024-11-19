#include <gtest/gtest.h>
#include "bytes.hpp"
#include "utils.hpp"
#include "clvm_value.hpp"
#include "clvm.hpp"
#include <vector>
#include <array>
#include <string>
#include <algorithm> // For std::equal
#include "coin.hpp"

class ChiaFFITest : public ::testing::Test
{
protected: // Change from private to protected
  std::array<uint8_t, 32> puzzleHash;
  std::string prefix;

  // Test data setup
  std::array<uint8_t, 32> test_data_32;
  std::array<uint8_t, 48> test_data_48;

  static std::string hex_encode(const ChiaBytes &bytes)
  {
    char *hex = to_hex(bytes.get());
    if (!hex)
      return "";
    std::string result(hex);
    free_string(hex);
    return result;
  }

  void SetUp() override
  {
    // Initialize test data
    puzzleHash.fill(0xAB); // Example hash filled with 0xAB
    prefix = "xch";
    for (size_t i = 0; i < 32; i++)
    {
      test_data_32[i] = static_cast<uint8_t>(i);
    }
    for (size_t i = 0; i < 48; i++)
    {
      test_data_48[i] = static_cast<uint8_t>(i + 100);
    }
  }

  void TearDown() override
  {
    // Cleanup, if necessary
  }

  bool starts_with(const std::string &str, const std::string &prefix)
  {
    return str.size() >= prefix.size() && std::equal(prefix.begin(), prefix.end(), str.begin());
  }
};

// TEST_F(ChiaFFITest, NFTMintAndSpend)
// {
//   // Initialize CLVM
//   ChiaClvm clvm;

//   // Create test P2 puzzle hash and keys
//   ChiaBytes32 p2_puzzle_hash = chia::hex_to_bytes32("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
//   ChiaBytes48 p2_public_key = chia::hex_to_bytes48("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");

//   // Create parent coin ID
//   ChiaBytes32 parent_coin_id = chia::hex_to_bytes32("9abcdef1234567890abcdef1234567890abcdef1234567890abcdef12345678");

//   // Create NFT metadata
//   ChiaNftMetadata metadata(1, 1); // edition_number, edition_total
//   metadata.set_data_uris({ChiaBytes::from_string("https://example.com")});
//   metadata.set_metadata_uris({ChiaBytes::from_string("https://example.com")});
//   metadata.set_license_uris({ChiaBytes::from_string("https://example.com")});

//   // Create NFT mint info
//   ChiaNftInfo mint_info(
//       ChiaBytes32(),                           // launcher_id (will be generated)
//       clvm.nft_metadata(metadata).serialize(), // metadata as bytes
//       NFT_METADATA_UPDATER_PUZZLE_HASH,        // standard updater hash
//       std::nullopt,                            // no current owner
//       p2_puzzle_hash,                          // royalty puzzle hash
//       300,                                     // royalty basis points
//       p2_puzzle_hash                           // p2 puzzle hash
//   );

// }

TEST_F(ChiaFFITest, CalculateCoinId)
{
  // Create input data from hex strings
  ChiaBytes32 parentCoinInfo = chia::hex_to_bytes32(
      "4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a");

  ChiaBytes32 puzzleHash = chia::hex_to_bytes32(
      "dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986");

  // Create the coin
  ChiaCoin coin = ChiaCoin::create(parentCoinInfo, puzzleHash, 100);

  // Get the coin ID
  ChiaBytes32 coinId = coin.coin_id();

  // Expected result
  ChiaBytes32 expectedCoinId = chia::hex_to_bytes32(
      "fd3e669c27be9d634fe79f1f7d7d8aaacc3597b855cffea1d708f4642f1d542a");

  // Compare the result
  EXPECT_EQ(coinId, expectedCoinId)
      << "Coin ID calculation did not match expected value";
}

// Construction and Basic Operation Tests
TEST_F(ChiaFFITest, DefaultConstruction)
{
  ChiaBytes32 bytes32;
  EXPECT_TRUE(bytes32.is_empty()) << "Default constructed ChiaBytes32 should be empty";

  ChiaBytes48 bytes48;
  EXPECT_TRUE(bytes48.is_empty()) << "Default constructed ChiaBytes48 should be empty";
}

TEST_F(ChiaFFITest, ArrayConstruction)
{
  ChiaBytes32 bytes32(test_data_32);
  EXPECT_FALSE(bytes32.is_empty());
  EXPECT_EQ(0, std::memcmp(bytes32.data(), test_data_32.data(), 32));

  ChiaBytes48 bytes48(test_data_48);
  EXPECT_FALSE(bytes48.is_empty());
  EXPECT_EQ(0, std::memcmp(bytes48.data(), test_data_48.data(), 48));
}

TEST_F(ChiaFFITest, FFITypeConversion)
{
  // Test conversion from FFI type
  Bytes32 ffi_bytes32{};
  std::memcpy(ffi_bytes32.data, test_data_32.data(), 32);
  ChiaBytes32 bytes32(ffi_bytes32);

  // Test conversion back to FFI type
  const Bytes32 &converted = bytes32.bytes();
  EXPECT_EQ(0, std::memcmp(converted.data, test_data_32.data(), 32));
}

// FFI Integration Tests
TEST_F(ChiaFFITest, PuzzleHashAddressRoundTrip)
{
  ChiaBytes32 original(test_data_32);
  std::string prefix = "xch";

  // Encode to address
  std::string address = chia::encode_puzzle_hash_to_address(original, prefix);
  EXPECT_FALSE(address.empty());
  EXPECT_TRUE(address.find(prefix) == 0) << "Address should start with prefix";

  // Decode back to puzzle hash
  ChiaBytes32 decoded = chia::decode_address_to_puzzle_hash(address);
  EXPECT_FALSE(decoded.is_empty());
  EXPECT_EQ(original, decoded) << "Round-trip conversion should preserve data";
}

TEST_F(ChiaFFITest, Bech32RoundTrip)
{
  ChiaBytes32 original(test_data_32);

  // Encode to bech32
  std::string encoded = chia::encode_puzzle_hash_bech32(original);
  EXPECT_FALSE(encoded.empty());

  // Decode back
  ChiaBytes32 decoded = chia::decode_puzzle_hash_bech32(encoded);
  EXPECT_FALSE(decoded.is_empty());
  EXPECT_EQ(original, decoded) << "Bech32 round-trip should preserve data";
}

// TEST_F(ChiaFFITest, InvalidInputHandling)
// {
//   // Test invalid address decoding
//   ChiaBytes32 result = chia::decode_address_to_puzzle_hash("invalid_address");
//   EXPECT_TRUE(result.is_empty()) << "Invalid address should result in empty bytes";

//   // Test invalid bech32 decoding
//   result = chia::decode_puzzle_hash_bech32("invalid_bech32");
//   EXPECT_TRUE(result.is_empty()) << "Invalid bech32 should result in empty bytes";
// }

// Array Conversion Tests
TEST_F(ChiaFFITest, ArrayConversion)
{
  ChiaBytes32 original(test_data_32);
  auto array = original.to_array();
  EXPECT_EQ(0, std::memcmp(array.data(), test_data_32.data(), 32));

  ChiaBytes32 reconstructed(array);
  EXPECT_EQ(original, reconstructed);
}

// Edge Cases
TEST_F(ChiaFFITest, NullPointerHandling)
{
  const uint8_t *null_ptr = nullptr;
  ChiaBytes32 bytes(null_ptr);
  EXPECT_TRUE(bytes.is_empty()) << "Construction with null pointer should result in empty bytes";
}

TEST_F(ChiaFFITest, DataConsistency)
{
  ChiaBytes32 bytes32(test_data_32);
  ChiaBytes48 bytes48(test_data_48);

  // Verify size constraints
  EXPECT_EQ(32u, ChiaBytes32::size());
  EXPECT_EQ(48u, ChiaBytes48::size());

  // Verify data consistency
  EXPECT_EQ(0, std::memcmp(bytes32.data(), test_data_32.data(), 32));
  EXPECT_EQ(0, std::memcmp(bytes48.data(), test_data_48.data(), 48));
}

// Comparison Operator Tests
TEST_F(ChiaFFITest, ComparisonOperators)
{
  ChiaBytes32 bytes1(test_data_32);
  ChiaBytes32 bytes2(test_data_32);
  ChiaBytes32 empty1;
  ChiaBytes32 empty2;

  EXPECT_EQ(bytes1, bytes2) << "Equal data should compare equal";
  EXPECT_EQ(empty1, empty2) << "Empty bytes should compare equal";
  EXPECT_NE(bytes1, empty1) << "Empty and non-empty bytes should not compare equal";
}

TEST_F(ChiaFFITest, encode_puzzle_hash_to_address_ValidInput)
{
  ChiaBytes32 puzzleHashWrapper(puzzleHash);
  std::string address = chia::encode_puzzle_hash_to_address(puzzleHashWrapper, prefix);
  EXPECT_FALSE(address.empty()) << "Address should not be empty.";
  EXPECT_TRUE(starts_with(address, prefix)) << "Address should start with the prefix.";
}

TEST_F(ChiaFFITest, decode_address_to_puzzle_hash_ValidAddress)
{
  ChiaBytes32 originalHash(puzzleHash);
  std::string address = chia::encode_puzzle_hash_to_address(originalHash, prefix);
  ChiaBytes32 decodedHash = chia::decode_address_to_puzzle_hash(address);

  EXPECT_EQ(decodedHash, originalHash)
      << "Decoded puzzle hash should match the original hash.";
}

TEST_F(ChiaFFITest, decode_address_to_puzzle_hash_InvalidAddress)
{
  std::string invalidAddress = "not-a-valid-address";
  ChiaBytes32 decodedHash = chia::decode_address_to_puzzle_hash(invalidAddress);

  EXPECT_TRUE(decodedHash.is_empty())
      << "Decoding an invalid address should result in an empty hash.";
}

TEST_F(ChiaFFITest, encode_puzzle_hash_bech32_ValidInput)
{
  ChiaBytes32 puzzleHashWrapper(puzzleHash);
  std::string encoded = chia::encode_puzzle_hash_bech32(puzzleHashWrapper);
  EXPECT_FALSE(encoded.empty()) << "Bech32-encoded puzzle hash should not be empty.";
}

TEST_F(ChiaFFITest, decode_puzzle_hash_bech32_ValidInput)
{
  ChiaBytes32 originalHash(puzzleHash);
  std::string encoded = chia::encode_puzzle_hash_bech32(originalHash);
  ChiaBytes32 decodedHash = chia::decode_puzzle_hash_bech32(encoded);

  EXPECT_EQ(decodedHash, originalHash)
      << "Decoded Bech32 puzzle hash should match the original hash.";
}

TEST_F(ChiaFFITest, decode_puzzle_hash_bech32_InvalidInput)
{
  std::string invalidEncoded = "not-a-valid-bech32";
  ChiaBytes32 decodedHash = chia::decode_puzzle_hash_bech32(invalidEncoded);

  EXPECT_TRUE(decodedHash.is_empty())
      << "Decoding an invalid Bech32 string should result in an empty hash.";
}

// TEST_F(ChiaFFITest, CreateFromArrayAndToArray)
// {
//   std::array<uint8_t, 32> data = {
//       0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11,
//       0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
//       0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11,
//       0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99};

//   ChiaBytes32 bytes32(data);

//   // Test if data is stored correctly
//   EXPECT_EQ(bytes32.to_array(), data);
// }

// TEST_F(ChiaFFITest, HexEncodingAndDecoding)
// {
//   std::string hex_str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
//   ChiaBytes32 bytes32 = ChiaBytes32::from_hex(hex_str);

//   // Verify encoding back to hex
//   EXPECT_EQ(ChiaBytes::hex_encode(bytes32.to_bytes()), hex_str);

//   // Convert to array and check values
//   std::array<uint8_t, 32> expected_data;
//   expected_data.fill(0xAA);
//   EXPECT_EQ(bytes32.to_array(), expected_data);
// }

// TEST_F(ChiaFFITest, MoveSemantics)
// {
//   std::vector<uint8_t> data = {0x01, 0x02};
//   ChiaBytes original(data);

//   // Move constructor
//   ChiaBytes moved(std::move(original));
//   EXPECT_EQ(moved.to_vector(), data);

//   // Check that original is empty after move
//   EXPECT_TRUE(original.to_vector().empty());

//   // Move assignment
//   ChiaBytes assigned;
//   assigned = std::move(moved);
//   EXPECT_EQ(assigned.to_vector(), data);

//   // Check that moved is empty after move
//   EXPECT_TRUE(moved.to_vector().empty());
// }

// TEST_F(ChiaFFITest, ChiaBytes32FromHex)
// {
//   // Expected data for parent_coin_info and puzzle_hash
//   std::array<uint8_t, 32> expected_parent_coin_info = {
//       0x4b, 0xf5, 0x12, 0x2f, 0x34, 0x45, 0x54, 0xc5,
//       0x3b, 0xde, 0x2e, 0xbb, 0x8c, 0xd2, 0xb7, 0xe3,
//       0xd1, 0x60, 0x0a, 0xd6, 0x31, 0xc3, 0x85, 0xa5,
//       0xd7, 0xcc, 0xe2, 0x3c, 0x77, 0x85, 0x45, 0x9a};

//   std::array<uint8_t, 32> expected_puzzle_hash = {
//       0xdb, 0xc1, 0xb4, 0xc9, 0x00, 0xff, 0xe4, 0x8d,
//       0x57, 0x5b, 0x5d, 0xa5, 0xc6, 0x38, 0x04, 0x01,
//       0x25, 0xf6, 0x5d, 0xb0, 0xfe, 0x3e, 0x24, 0x49,
//       0x4b, 0x76, 0xea, 0x98, 0x64, 0x57, 0xd9, 0x86};

//   // Create ChiaBytes32 objects using from_hex
//   ChiaBytes32 parent_coin_info = ChiaBytes32::from_hex("4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a");
//   ChiaBytes32 puzzle_hash = ChiaBytes32::from_hex("dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986");

//   // Check that the resulting byte arrays match the expected values
//   EXPECT_EQ(parent_coin_info.to_array(), expected_parent_coin_info);
//   EXPECT_EQ(puzzle_hash.to_array(), expected_puzzle_hash);
// }

// TEST_F(ChiaFFITest, CreateFromArrayAndToArray32)
// {
//   std::array<uint8_t, 32> data = {
//       0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11,
//       0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
//       0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11,
//       0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99};

//   ChiaBytes32 bytes32(data);

//   // Test if data is stored correctly
//   EXPECT_EQ(bytes32.to_array(), data);
// }

// TEST_F(ChiaFFITest, HexEncodingAndDecoding32)
// {
//   std::string hex_str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
//   ChiaBytes32 bytes32 = ChiaBytes32::from_hex(hex_str);

//   // Verify encoding back to hex
//   EXPECT_EQ(ChiaBytes::hex_encode(bytes32.to_bytes()), hex_str);

//   // Convert to array and check values
//   std::array<uint8_t, 32> expected_data;
//   expected_data.fill(0xAA);
//   EXPECT_EQ(bytes32.to_array(), expected_data);
// }

// TEST_F(ChiaFFITest, ConversionBetweenChiaBytesAndChiaBytes32)
// {
//   // Initial data for ChiaBytes32
//   std::array<uint8_t, 32> data = {
//       0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
//       0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0x00,
//       0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
//       0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x01};

//   // Create ChiaBytes32 and convert to ChiaBytes
//   ChiaBytes32 bytes32(data);
//   ChiaBytes bytes = bytes32.to_bytes();

//   // Convert back from ChiaBytes to ChiaBytes32
//   ChiaBytes32 converted_back = ChiaBytes32::from_bytes(bytes);

//   // Verify the conversion matches original data
//   EXPECT_EQ(converted_back.to_array(), data);
// }

// TEST_F(ChiaFFITest, MoveSemantics32)
// {
//   std::array<uint8_t, 32> data;
//   data.fill(0x11);
//   ChiaBytes32 original(data);

//   // Move constructor
//   ChiaBytes32 moved(std::move(original));
//   EXPECT_EQ(moved.to_array(), data);

//   // Check that original is empty after move
//   std::array<uint8_t, 32> empty_array{};
//   EXPECT_EQ(original.to_array(), empty_array);

//   // Move assignment
//   ChiaBytes32 assigned;
//   assigned = std::move(moved);
//   EXPECT_EQ(assigned.to_array(), data);

//   // Check that moved is empty after move
//   EXPECT_EQ(moved.to_array(), empty_array);
// }

// TEST_F(ChiaFFITest, CalculateCoinId)
// {
//   const std::array<uint8_t, 32> parent_info = {
//       0x4b, 0xf5, 0x12, 0x2f, 0x34, 0x45, 0x54, 0xc5,
//       0x3b, 0xde, 0x2e, 0xbb, 0x8c, 0xd2, 0xb7, 0xe3,
//       0xd1, 0x60, 0x0a, 0xd6, 0x31, 0xc3, 0x85, 0xa5,
//       0xd7, 0xcc, 0xe2, 0x3c, 0x77, 0x85, 0x45, 0x9a};

//   const std::array<uint8_t, 32> puzzle_hash = {
//       0xdb, 0xc1, 0xb4, 0xc9, 0x00, 0xff, 0xe4, 0x8d,
//       0x57, 0x5b, 0x5d, 0xa5, 0xc6, 0x38, 0x04, 0x01,
//       0x25, 0xf6, 0x5d, 0xb0, 0xfe, 0x3e, 0x24, 0x49,
//       0x4b, 0x76, 0xea, 0x98, 0x64, 0x57, 0xd9, 0x86};

//   const uint64_t amount = 100;

//   auto coin = ChiaCoin::create(parent_info, puzzle_hash, amount);

//   EXPECT_EQ(coin.get_parent_coin_info(), parent_info);
//   EXPECT_EQ(coin.get_puzzle_hash(), puzzle_hash);
//   EXPECT_EQ(coin.get_amount(), amount);

//   const std::array<uint8_t, 32> expected_coin_id = {
//       0xfd, 0x3e, 0x66, 0x9c, 0x27, 0xbe, 0x9d, 0x63,
//       0x4f, 0xe7, 0x9f, 0x1f, 0x7d, 0x7d, 0x8a, 0xaa,
//       0xcc, 0x35, 0x97, 0xb8, 0x55, 0xcf, 0xfe, 0xa1,
//       0xd7, 0x08, 0xf4, 0x64, 0x2f, 0x1d, 0x54, 0x2a};

//   EXPECT_EQ(coin.get_coin_id(), expected_coin_id);
// }
// TEST_F(ChiaFFITest, CalculateCoinId)
// {
//   // Initialize a ChiaClvm instance
//   ChiaClvm clvm;

//   // Define known input values for parent_coin_info, puzzle_hash, and amount
//   ChiaBytes32 parent_coin_info = ChiaBytes32::from_hex("4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a");
//   ChiaBytes32 puzzle_hash = ChiaBytes32::from_hex("dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986");
//   uint64_t amount = 100;

//   std::cout << "Parent Coin Info: " << ChiaBytes::hex_encode(parent_coin_info.to_bytes()) << std::endl;
//   std::cout << "Puzzle Hash: " << ChiaBytes::hex_encode(puzzle_hash.to_bytes()) << std::endl;
//   std::cout << "Amount: " << amount << std::endl;
//   // Create a ChiaCoin object
//   ChiaCoin coin = ChiaCoin::create(parent_coin_info, puzzle_hash, amount);

//   // Verify and log each field
//   EXPECT_EQ(coin.get_amount(), amount);
//   std::cout << "Expected Amount: " << amount << ", Retrieved Amount: " << coin.get_amount() << std::endl;

//   EXPECT_EQ(coin.get_parent_coin_info().to_array(), parent_coin_info.to_array());
//   std::cout << "Expected Parent Coin Info: " << ChiaBytes::hex_encode(parent_coin_info.to_bytes())
//             << ", Retrieved Parent Coin Info: " << ChiaBytes::hex_encode(coin.get_parent_coin_info().to_bytes()) << std::endl;

//   EXPECT_EQ(coin.get_puzzle_hash().to_array(), puzzle_hash.to_array());
//   std::cout << "Expected Puzzle Hash: " << ChiaBytes::hex_encode(puzzle_hash.to_bytes())
//             << ", Retrieved Puzzle Hash: " << ChiaBytes::hex_encode(coin.get_puzzle_hash().to_bytes()) << std::endl;

//   // Calculate the coin ID using get_coin_id method
//   ChiaBytes32 calculated_coin_id = coin.get_coin_id();

//   // Expected coin ID
//   ChiaBytes32 expected_coin_id = ChiaBytes32::from_hex("fd3e669c27be9d634fe79f1f7d7d8aaacc3597b855cffea1d708f4642f1d542a");

//   std::cout << "Expected Coin ID: " << ChiaBytes::hex_encode(expected_coin_id.to_bytes()) << std::endl;
//   std::cout << "Calculated Coin ID: " << ChiaBytes::hex_encode(calculated_coin_id.to_bytes()) << std::endl;

//   // Compare the calculated coin ID with the expected value
//   EXPECT_EQ(calculated_coin_id.to_array(), expected_coin_id.to_array());
// }

// TEST_F(ChiaFFITest, BytesWrapperBasicOperations)
// {
//   // Test empty constructor
//   ChiaBytes empty;
//   EXPECT_EQ(empty.to_vector().size(), 0);

//   // Test with actual data
//   std::vector<uint8_t> test_data = {1, 2, 3, 4};
//   ChiaBytes bytes(test_data);

//   // Test to_vector
//   auto result = bytes.to_vector();
//   ASSERT_EQ(result.size(), test_data.size());
//   EXPECT_EQ(result, test_data);

//   // Test clone
//   auto cloned = bytes.clone();
//   EXPECT_EQ(cloned.to_vector(), test_data);

//   // Test move constructor
//   ChiaBytes moved(std::move(cloned));
//   EXPECT_EQ(moved.to_vector(), test_data);

//   // Test move assignment
//   ChiaBytes assigned;
//   assigned = std::move(moved);
//   EXPECT_EQ(assigned.to_vector(), test_data);
// }

// TEST_F(ChiaFFITest, Bytes32WrapperBasicOperations)
// {
//   // Test empty constructor
//   ChiaBytes32 empty;
//   auto empty_array = empty.to_array();
//   for (auto byte : empty_array)
//   {
//     EXPECT_EQ(byte, 0);
//   }

//   // Test with actual data
//   std::array<uint8_t, 32> test_data;
//   for (size_t i = 0; i < 32; i++)
//   {
//     test_data[i] = i;
//   }

//   ChiaBytes32 bytes32(test_data);

//   // Test to_array
//   auto result = bytes32.to_array();
//   EXPECT_EQ(result, test_data);

//   // Test move constructor
//   ChiaBytes32 moved(std::move(bytes32));
//   EXPECT_EQ(moved.to_array(), test_data);

//   // Test move assignment
//   ChiaBytes32 assigned;
//   assigned = std::move(moved);
//   EXPECT_EQ(assigned.to_array(), test_data);
// }

// TEST_F(ChiaFFITest, ConversionsBetweenTypes)
// {
//   // Create test data
//   std::array<uint8_t, 32> test_data;
//   for (size_t i = 0; i < 32; i++)
//   {
//     test_data[i] = i;
//   }

//   // Test Bytes32 to Bytes conversion
//   ChiaBytes32 bytes32(test_data);
//   ChiaBytes converted = bytes32.to_bytes();

//   auto result = converted.to_vector();
//   ASSERT_EQ(result.size(), 32);
//   for (size_t i = 0; i < 32; i++)
//   {
//     EXPECT_EQ(result[i], test_data[i]);
//   }

//   // Test Bytes to Bytes32 conversion
//   auto converted_back = ChiaBytes32::from_bytes(converted);
//   EXPECT_EQ(converted_back.to_array(), test_data);
// }

// // TEST_F(ChiaFFITest, NullHandling)
// // {
// //   // Test null CBytes
// //   ChiaBytes null_bytes(nullptr);
// //   EXPECT_EQ(null_bytes.to_vector().size(), 0);

// //   // Test null CBytes32
// //   ChiaBytes32 null_bytes32(nullptr);
// //   auto null_array = null_bytes32.to_array();
// //   for (auto byte : null_array)
// //   {
// //     EXPECT_EQ(byte, 0);
// //   }
// // }

// TEST_F(ChiaFFITest, ReleaseAndGet)
// {
//   std::vector<uint8_t> test_data = {1, 2, 3, 4};
//   ChiaBytes bytes(test_data);

//   // Test get
//   CBytes *handle = bytes.get();
//   ASSERT_NE(handle, nullptr);

//   // Test release
//   CBytes *released = bytes.release();
//   ASSERT_NE(released, nullptr);
//   EXPECT_EQ(bytes.get(), nullptr); // Should be null after release

//   // Clean up
//   c_bytes_free(released);
// }

// TEST_F(ChiaFFITest, EdgeCases)
// {
//   // Test empty vector
//   ChiaBytes empty_vec(std::vector<uint8_t>{});
//   EXPECT_EQ(empty_vec.to_vector().size(), 0);

//   // Test single byte
//   ChiaBytes single_byte(std::vector<uint8_t>{42});
//   auto result = single_byte.to_vector();
//   ASSERT_EQ(result.size(), 1);
//   EXPECT_EQ(result[0], 42);

//   // Test large data
//   std::vector<uint8_t> large_data(1000, 0xFF);
//   ChiaBytes large_bytes(large_data);
//   EXPECT_EQ(large_bytes.to_vector(), large_data);
// }

// TEST_F(ChiaFFITest, Bytes32Initialization)
// {
//   // Test zero initialization
//   ChiaBytes32 zero;
//   auto zero_array = zero.to_array();
//   for (auto byte : zero_array)
//   {
//     EXPECT_EQ(byte, 0);
//   }

//   // Test specific pattern
//   std::array<uint8_t, 32> pattern;
//   for (size_t i = 0; i < 32; i++)
//   {
//     pattern[i] = (i * 2) % 256;
//   }

//   ChiaBytes32 patterned(pattern);
//   EXPECT_EQ(patterned.to_array(), pattern);
// }

// // Optional: Performance test (disabled by default)
// TEST_F(ChiaFFITest, DISABLED_Performance)
// {
//   const int iterations = 10000;

//   // Test ChiaBytes performance
//   std::vector<uint8_t> test_data(1000, 0xFF);
//   for (int i = 0; i < iterations; i++)
//   {
//     ChiaBytes bytes(test_data);
//     auto vec = bytes.to_vector();
//     ASSERT_EQ(vec.size(), test_data.size());
//   }

//   // Test ChiaBytes32 performance
//   std::array<uint8_t, 32> test_array;
//   for (int i = 0; i < iterations; i++)
//   {
//     ChiaBytes32 bytes32(test_array);
//     auto arr = bytes32.to_array();
//     ASSERT_EQ(arr.size(), 32);
//   }
// }

// TEST_F(ChiaFFITest, SHA256Operations)
// {
//   // Test empty input
//   ChiaBytes empty;
//   ChiaBytes empty_hash = empty.sha256();
//   ASSERT_EQ(empty_hash.to_vector().size(), 32); // SHA256 always produces 32 bytes

//   // Test known input with known output
//   // "abc" => ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
//   std::vector<uint8_t> input = {'a', 'b', 'c'};
//   ChiaBytes bytes(input);
//   ChiaBytes hash = bytes.sha256();

//   auto hash_result = hash.to_vector();
//   ASSERT_EQ(hash_result.size(), 32);

//   // Expected hash for "abc"
//   std::vector<uint8_t> expected = {
//       0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea,
//       0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23,
//       0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c,
//       0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad};
//   EXPECT_EQ(hash_result, expected);

//   // Test larger input
//   std::vector<uint8_t> large_input(1000, 0xFF);
//   ChiaBytes large_bytes(large_input);
//   ChiaBytes large_hash = large_bytes.sha256();
//   ASSERT_EQ(large_hash.to_vector().size(), 32);

//   // Test null handling
//   ChiaBytes null_bytes(nullptr);
//   ChiaBytes null_hash = null_bytes.sha256();
//   EXPECT_EQ(null_hash.to_vector().size(), 32); // Should still return a valid hash
// }

// TEST_F(ChiaFFITest, AddressEncodingDecoding)
// {
//   // Known test vector - can replace with actual Chia test vectors
//   std::array<uint8_t, 32> test_puzzle_hash = {
//       0xac, 0x4a, 0x90, 0xe9, 0xf3, 0xeb, 0xca, 0xfa,
//       0x3d, 0x53, 0x42, 0xd3, 0x47, 0xdb, 0x27, 0x03,
//       0xb3, 0x10, 0x29, 0x51, 0x1f, 0x5b, 0x40, 0xc1,
//       0x14, 0x41, 0xaf, 0x1c, 0x96, 0x1f, 0x65, 0x85};

//   // Test XCH address encoding/decoding
//   std::string xch_address = ChiaAddressWrapper::chia::encode_puzzle_hash_to_address(test_puzzle_hash, "xch");
//   ASSERT_FALSE(xch_address.empty());
//   EXPECT_EQ(xch_address.substr(0, 3), "xch");

//   // Test roundtrip conversion
//   ChiaBytes32 decoded_puzzle_hash = ChiaAddressWrapper::chia::decode_address_to_puzzle_hash(xch_address);
//   EXPECT_EQ(decoded_puzzle_hash.to_array(), test_puzzle_hash);
// }

// TEST_F(ChiaFFITest, AddressEdgeCases)
// {
//   // Test with zero puzzle hash
//   std::array<uint8_t, 32> zero_puzzle_hash = {};
//   std::string zero_address = ChiaAddressWrapper::chia::encode_puzzle_hash_to_address(zero_puzzle_hash, "xch");
//   ASSERT_FALSE(zero_address.empty());

//   ChiaBytes32 decoded_zero = ChiaAddressWrapper::chia::decode_address_to_puzzle_hash(zero_address);
//   EXPECT_EQ(decoded_zero.to_array(), zero_puzzle_hash);

//   // Test with invalid address
//   ChiaBytes32 invalid_result = ChiaAddressWrapper::chia::decode_address_to_puzzle_hash("invalid_address");
//   std::array<uint8_t, 32> zero_array = {};
//   EXPECT_EQ(invalid_result.to_array(), zero_array);

//   // Test with empty prefix
//   std::array<uint8_t, 32> test_puzzle_hash = {};
//   for (size_t i = 0; i < 32; i++)
//   {
//     test_puzzle_hash[i] = i;
//   }
//   std::string empty_prefix_address = ChiaAddressWrapper::chia::encode_puzzle_hash_to_address(test_puzzle_hash, "");
//   EXPECT_TRUE(empty_prefix_address.empty()); // Should handle empty prefix gracefully
// }

// TEST_F(ChiaFFITest, AddressDifferentPrefixes)
// {
//   std::array<uint8_t, 32> test_puzzle_hash = {};
//   for (size_t i = 0; i < 32; i++)
//   {
//     test_puzzle_hash[i] = i;
//   }

//   // Test different prefixes
//   std::vector<std::string> prefixes = {"xch", "txch", "txch"};

//   for (const auto &prefix : prefixes)
//   {
//     std::string address = ChiaAddressWrapper::chia::encode_puzzle_hash_to_address(test_puzzle_hash, prefix);
//     ASSERT_FALSE(address.empty());
//     EXPECT_EQ(address.substr(0, prefix.length()), prefix);

//     // Verify roundtrip
//     ChiaBytes32 decoded = ChiaAddressWrapper::chia::decode_address_to_puzzle_hash(address);
//     EXPECT_EQ(decoded.to_array(), test_puzzle_hash);
//   }
// }

// TEST_F(ChiaFFITest, Bech32EncodingEdgeCases)
// {
//   // Test with zero puzzle hash
//   std::array<uint8_t, 32> zero_puzzle_hash = {};
//   std::string zero_encoded = ChiaAddressWrapper::chia::encode_puzzle_hash_bech32(zero_puzzle_hash);
//   ASSERT_FALSE(zero_encoded.empty());

//   // Verify roundtrip
//   ChiaBytes32 decoded_zero = ChiaAddressWrapper::chia::decode_puzzle_hash_bech32(zero_encoded);
//   EXPECT_EQ(decoded_zero.to_array(), zero_puzzle_hash);

//   // Test with invalid bech32 string
//   ChiaBytes32 invalid_result = ChiaAddressWrapper::chia::decode_puzzle_hash_bech32("invalid_bech32");
//   std::array<uint8_t, 32> zero_array = {};
//   EXPECT_EQ(invalid_result.to_array(), zero_array);

//   // Test with empty string
//   ChiaBytes32 empty_result = ChiaAddressWrapper::chia::decode_puzzle_hash_bech32("");
//   EXPECT_EQ(empty_result.to_array(), zero_array);
// }

// TEST_F(ChiaFFITest, HexConversionOperations)
// {
//   // Test basic hex conversion
//   std::vector<uint8_t> test_data = {0xde, 0xad, 0xbe, 0xef};
//   ChiaBytes bytes(test_data);

//   // Get hex string
//   std::string hex = ChiaBytes::hex_encode(bytes);
//   EXPECT_EQ(hex, "deadbeef");

//   // Convert back from hex
//   ChiaBytes decoded = ChiaBytes::from_hex(hex);
//   EXPECT_EQ(decoded.to_vector(), test_data);

//   // Test with 0x prefix
//   ChiaBytes decoded_with_prefix = ChiaBytes::from_hex("0xdeadbeef");
//   EXPECT_EQ(decoded_with_prefix.to_vector(), test_data);
// }

// TEST_F(ChiaFFITest, HexEdgeCases)
// {
//   // Test empty hex string
//   ChiaBytes empty = ChiaBytes::from_hex("");
//   EXPECT_EQ(empty.to_vector().size(), 0);

//   // Test invalid hex characters
//   ChiaBytes invalid = ChiaBytes::from_hex("xyz123");
//   EXPECT_EQ(invalid.to_vector().size(), 0);

//   // Test odd length hex string
//   ChiaBytes odd_length = ChiaBytes::from_hex("abc");
//   EXPECT_EQ(odd_length.to_vector().size(), 0);

//   // Test null terminator in middle
//   ChiaBytes null_middle = ChiaBytes::from_hex("dead\0beef");
//   EXPECT_EQ(null_middle.to_vector().size(), 2); // Should only process up to null

//   // Test uppercase hex
//   std::vector<uint8_t> expected = {0xDE, 0xAD, 0xBE, 0xEF};
//   ChiaBytes uppercase = ChiaBytes::from_hex("DEADBEEF");
//   EXPECT_EQ(uppercase.to_vector(), expected);
// }

// TEST_F(ChiaFFITest, BytesComparison)
// {
//   // Test equal bytes
//   std::vector<uint8_t> data1 = {1, 2, 3, 4};
//   std::vector<uint8_t> data2 = {1, 2, 3, 4};
//   ChiaBytes bytes1(data1);
//   ChiaBytes bytes2(data2);
//   EXPECT_TRUE(bytes1 == bytes2);

//   // Test different length
//   std::vector<uint8_t> data3 = {1, 2, 3};
//   ChiaBytes bytes3(data3);
//   EXPECT_FALSE(bytes1 == bytes3);

//   // Test different content
//   std::vector<uint8_t> data4 = {1, 2, 3, 5};
//   ChiaBytes bytes4(data4);
//   EXPECT_FALSE(bytes1 == bytes4);

//   // Test with empty bytes
//   ChiaBytes empty1;
//   ChiaBytes empty2;
//   EXPECT_TRUE(empty1 == empty2);

//   // Test empty with non-empty
//   EXPECT_FALSE(empty1 == bytes1);

//   // Test null handles
//   ChiaBytes null1(nullptr);
//   ChiaBytes null2(nullptr);
//   EXPECT_TRUE(null1 == null2);
//   EXPECT_FALSE(null1 == bytes1);
// }

// TEST_F(ChiaFFITest, AtomRoundtrip)
// {
//   ChiaClvm clvm;
//   std::vector<uint8_t> expected = {1, 2, 3};

//   ChiaProgram atom = clvm.alloc_value(expected);
//   ASSERT_TRUE(atom.get());

//   ChiaBytes result = atom.to_atom();
//   EXPECT_EQ(result.to_vector(), expected);
// }

// TEST_F(ChiaFFITest, StringRoundtrip)
// {
//   ChiaClvm clvm;
//   std::string expected = "hello world";

//   ChiaProgram atom = clvm.alloc_string(expected);
//   ASSERT_TRUE(atom.get());

//   ChiaBytes result = atom.to_atom();
//   std::string actual(reinterpret_cast<const char *>(result.data()), result.size());
//   EXPECT_EQ(actual, expected);
// }

// TEST_F(ChiaFFITest, NumberRoundtrip)
// {
//   ChiaClvm clvm;
//   const std::vector<double> test_numbers = {
//       -9007199254740991.0,
//       -1000.0,
//       0.0,
//       34.0,
//       1000.0,
//       9007199254740991.0};

//   for (double expected : test_numbers)
//   {
//     ChiaProgram num = clvm.alloc_number(expected);
//     ASSERT_TRUE(num.get());
//   }
// }

// TEST_F(ChiaFFITest, InvalidNumber)
// {
//   ChiaClvm clvm;
//   const std::vector<double> invalid_numbers = {
//       -9007199254740992.0,
//       9007199254740992.0,
//       INFINITY,
//       -INFINITY,
//       NAN};

//   for (double invalid : invalid_numbers)
//   {
//     ChiaProgram num = clvm.alloc_number(invalid);
//     ASSERT_FALSE(num.get());
//   }
// }

// TEST_F(ChiaFFITest, BigIntRoundtrip)
// {
//   ChiaClvm clvm;
//   const std::vector<std::pair<std::vector<uint8_t>, int64_t>> test_cases = {
//       {{0x00}, 0},
//       {{0x01}, 1},
//       {{0x01, 0xA4}, 420},
//       {{0x03, 0xFF, 0xFF, 0xFF}, 67108863},
//       {{0xFF}, -1},
//       {{0x9C}, -100}};

//   for (const auto &[bytes, expected] : test_cases)
//   {
//     ChiaProgram num = clvm.alloc_bigint(bytes);
//     ASSERT_TRUE(num.get());
//   }
// }

// TEST_F(ChiaFFITest, PairRoundtrip)
// {
//   ChiaClvm clvm;
//   auto first_value = ClvmValueWrapper::from_number(1);
//   std::vector<uint8_t> bigint_bytes = {0x64};
//   auto rest_value = ClvmValueWrapper::from_bigint(bigint_bytes);

//   ChiaProgram pair = clvm.pair(first_value, rest_value);
//   ASSERT_TRUE(pair.get());
//   EXPECT_TRUE(pair.is_pair());

//   ChiaProgram first = pair.first();
//   ChiaProgram rest = pair.rest();
//   ASSERT_TRUE(first.get());
//   ASSERT_TRUE(rest.get());
// }

// // Added test for vector operations with ChiaBytes
// TEST_F(ChiaFFITest, ChiaBytesVectorOperations)
// {
//   // Test vector creation
//   std::vector<ChiaBytes> bytes_vec;

//   // Test emplace_back
//   bytes_vec.emplace_back(ChiaBytes::from_hex("deadbeef"));
//   EXPECT_EQ(bytes_vec.size(), 1);

//   // Test moving elements
//   ChiaBytes movable = ChiaBytes::from_hex("cafebabe");
//   bytes_vec.push_back(std::move(movable));
//   EXPECT_EQ(bytes_vec.size(), 2);

//   // Test vector clear
//   bytes_vec.clear();
//   EXPECT_EQ(bytes_vec.size(), 0);

//   // Test reserve and capacity
//   bytes_vec.reserve(10);
//   EXPECT_GE(bytes_vec.capacity(), 10);

//   // Test multiple emplace operations
//   bytes_vec.emplace_back(ChiaBytes::from_hex("11223344"));
//   bytes_vec.emplace_back(ChiaBytes::from_hex("55667788"));
//   EXPECT_EQ(bytes_vec.size(), 2);
// }

// TEST_F(ChiaFFITest, CurryTreeHash)
// {
//   ChiaClvm clvm;
//   ChiaProgram nil_program = clvm.alloc_nil();
//   ASSERT_TRUE(nil_program.get());

//   std::vector<ChiaProgram> items;
//   for (int i = 0; i < 10; i++)
//   {
//     items.push_back(clvm.alloc_number(i));
//   }

//   ChiaProgram curried = clvm.curry(nil_program, items);
//   ASSERT_TRUE(curried.get());

//   ChiaBytes32 nil_hash = nil_program.tree_hash();
//   std::vector<ChiaBytes32> arg_hashes;
//   for (const auto &item : items)
//   {
//     arg_hashes.push_back(item.tree_hash());
//   }

//   ChiaBytes32 result;
//   EXPECT_TRUE(clvm.curry_tree_hash(nil_hash, arg_hashes, result));
//   EXPECT_EQ(result.to_array(), curried.tree_hash().to_array());
// }

// TEST_F(ChiaFFITest, CreateAndParseCreateCoin)
// {
//   ChiaClvm clvm;
//   std::string hex_data(64, 'f');

//   ChiaBytes puzzle_hash_bytes = ChiaBytes::from_hex(hex_data);
//   ChiaBytes32 puzzle_hash = ChiaBytes32::from_bytes(puzzle_hash_bytes);
//   uint64_t amount = 1;
//   ChiaBytes memo = ChiaBytes::from_hex(hex_data);

//   std::vector<ChiaBytes> memos;
//   memos.push_back(std::move(memo));

//   std::cout << "Original puzzle_hash: " << ChiaBytes::hex_encode(puzzle_hash.to_bytes()) << std::endl;

//   auto coin_program = clvm.createCoin(puzzle_hash, amount, memos);
//   auto parsed = clvm.parseCreateCoin(coin_program);

//   std::cout << "Parsed puzzle_hash: " << ChiaBytes::hex_encode(parsed.puzzle_hash.to_bytes()) << std::endl;

//   EXPECT_EQ(parsed.puzzle_hash.to_array(), puzzle_hash.to_array());
//   EXPECT_EQ(parsed.amount, amount);
//   ASSERT_EQ(parsed.memos.size(), 1);
//   EXPECT_EQ(parsed.memos[0].to_vector(), memos[0].to_vector());
// }

// // // Test 1 : Basic Memory Leak Test with Stress
// TEST_F(ChiaFFITest, StressCreateAndParseCreateCoin)
// {
//   const int ITERATIONS = 1000;

//   for (int i = 0; i < ITERATIONS; ++i)
//   {
//     ChiaClvm clvm;
//     std::string hex_data(64, 'f');

//     ChiaBytes puzzle_hash_bytes = ChiaBytes::from_hex(hex_data);
//     ChiaBytes32 puzzle_hash = ChiaBytes32::from_bytes(puzzle_hash_bytes);
//     ChiaBytes memo = ChiaBytes::from_hex(hex_data);

//     std::vector<ChiaBytes> memos;
//     memos.push_back(std::move(memo));

//     auto parsed = clvm.parseCreateCoin(clvm.createCoin(puzzle_hash, i, memos));

//     EXPECT_EQ(parsed.amount, static_cast<uint64_t>(i));
//     EXPECT_EQ(parsed.puzzle_hash.to_array(), puzzle_hash.to_array());
//   }
// }
