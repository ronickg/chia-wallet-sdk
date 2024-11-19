#pragma once

#include <cstdint> // For fixed-width integer types like int8_t, uint64_t

namespace clvm
{

    // Stub for NodePtr
    struct NodePtr
    {
        static NodePtr NIL;   // Represents a NIL constant
        bool is_atom() const; // Checks if this NodePtr is an atom
        bool is_pair() const; // Checks if this NodePtr is a pair
    };

    // Stub for SpendContext
    class SpendContext
    {
    public:
        NodePtr allocator; // Allocator object associated with the context

        SpendContext();  // Constructor
        ~SpendContext(); // Destructor

        // Additional methods if needed
    };

} // namespace clvm
