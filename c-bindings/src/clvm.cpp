// clvm.cpp
#include "clvm.hpp"
#include "program.hpp"

namespace chia
{

    Clvm::Clvm() : pImpl(std::make_unique<Impl>()) {}
    Clvm::~Clvm() = default;

    void Clvm::setCleanupCallback(std::function<void()> callback)
    {
        pImpl->cleanup = std::move(callback);
    }

    ClvmHandle *Clvm::raw_handle() const
    {
        return pImpl->handle;
    }

    Program Clvm::allocate(const chia::ClvmValue &value) const
    {
        auto ptr = clvm_allocate(raw_handle(), value.raw_handle());
        if (!ptr)
        {
            throw std::runtime_error("Failed to allocate CLVM value");
        }
        return Program(*this, ptr);
    }

} // namespace chia