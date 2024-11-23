#ifndef CHIA_WALLET_FFI_H
#define CHIA_WALLET_FFI_H

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

typedef struct {
    uint8_t data[32];
} Bytes32;
typedef struct {
    uint8_t data[48];
} Bytes48;
typedef struct {
    uint8_t data[96];
} Bytes96;
typedef struct {
    uint8_t data[100];
} Bytes100;

typedef struct Simulator Simulator;

template<typename T>
struct Option {
    bool is_present;  // Corresponds to Rust's discriminant
    T value;         // The actual value storage

    // Default constructor creates None
    Option() : is_present(false) {}

    // Create Some variant
    static Option<T> some(const T& val) {
        Option<T> opt;
        opt.is_present = true;
        opt.value = val;
        return opt;
    }

    // Create None variant
    static Option<T> none() {
        return Option<T>();
    }

    bool has_value() const {
        return is_present;
    }

    const T& unwrap() const {
        if (!is_present) {
            throw std::runtime_error("Attempted to unwrap None value");
        }
        return value;
    }
};

// Allow conversion from nullptr to Option<T*>
template<typename T>
Option<T*> from_nullable(T* ptr) {
    if (ptr == nullptr) {
        return Option<T*>::none();
    }
    return Option<T*>::some(ptr);
}

struct Coin {
    Bytes32 parent_coin_info;
    Bytes32 puzzle_hash;
    uint64_t amount;
};

struct NodePtr;
struct ClvmValue;



enum class ClvmValueType {
  Number,
  BigInt,
  String,
  Array,
};

struct ClvmHandle;

struct ProgramHandle;

struct BytesHandle {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
};

struct ClvmValueArray {
  ClvmValue *items;
  size_t len;
};

union ClvmValueData {
  double number;
  BytesHandle *bigint;
  bool boolean;
  char *string;
  ProgramHandle *program;
  BytesHandle *bytes;
  ClvmValueArray *array;
};

struct ClvmValue {
  ClvmValueType value_type;
  ClvmValueData data;
};

extern "C" {

BytesHandle *bytes_create();

void bytes_destroy(BytesHandle *handle);

BytesHandle *bytes_from_slice(const uint8_t *data, uintptr_t len);

bool bytes_copy_to(const BytesHandle *handle, uint8_t *out, uintptr_t out_len);

uintptr_t bytes_len(const BytesHandle *handle);

ClvmHandle *clvm_create();

ProgramHandle *clvm_nil_program(const ClvmHandle *handle);

ProgramHandle *clvm_allocate(const ClvmHandle *handle, const ClvmValue *value);

void clvm_destroy(ClvmHandle *ptr);

void clvm_array_destroy(ClvmValueArray *array);

ClvmValueArray *clvm_array_from_values(const ClvmValue *values, size_t len);

void clvm_value_free(ClvmValue *value);

ProgramHandle *program_new(NodePtr ptr);

void program_destroy(ProgramHandle *ptr);

bool program_is_atom(const ProgramHandle *program);

bool program_is_pair(const ProgramHandle *program);

Bytes32 program_tree_hash(const ClvmHandle *handle, const ProgramHandle *program);

char *program_to_string(const ClvmHandle *handle, const ProgramHandle *program);

double program_to_number(const ClvmHandle *handle, const ProgramHandle *program);

BytesHandle *program_to_bigint_bytes(const ClvmHandle *handle, const ProgramHandle *program);

ProgramHandle *program_first(const ClvmHandle *handle, const ProgramHandle *program);

ProgramHandle *program_rest(const ClvmHandle *handle, const ProgramHandle *program);

void string_destroy(char *ptr);

}  // extern "C"

#endif  // CHIA_WALLET_FFI_H
