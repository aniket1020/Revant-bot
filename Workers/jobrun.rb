#!/usr/bin/env ruby

# TARGET_GID = 99
TARGET_UID = 0

COMMAND = ARGV[0]

# Process::Sys.setgid(TARGET_GID)
# Process::Sys.seteuid(TARGET_UID)

$SAFE = 4

Process::setrlimit(Process::RLIMIT_DATA, 1024 * 1024 * 256 * 4)   # 1GB Memory alloc
Process::setrlimit(Process::RLIMIT_CPU, 5, 10)                    # 5s Runtime

Process::exec(COMMAND)
