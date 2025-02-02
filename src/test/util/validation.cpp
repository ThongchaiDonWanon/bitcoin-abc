// Copyright (c) 2020 The Bitcoin Core developers
// Distributed under the MIT software license, see the accompanying
// file COPYING or http://www.opensource.org/licenses/mit-license.php.

#include <test/util/validation.h>

#include <validation.h>
#include <validationinterface.h>

void ValidationInterfaceTest::BlockConnected(
    CValidationInterface &obj, const std::shared_ptr<const CBlock> &block,
    const CBlockIndex *pindex) {
    obj.BlockConnected(block, pindex);
}
