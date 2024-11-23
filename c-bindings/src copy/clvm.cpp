// clvm.cpp
#include "clvm.hpp"

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