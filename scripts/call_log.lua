if tastudio then -- bizhawk
    shift_right = bit.rshift
    bitwise_and = bit.band
    bitwise_xor = bit.bxor
    read_byte = memory.read_u8
    read_word = memory.read_u16_le
    get_cycles = emu.totalexecutedcycles
    function get_program_counter()
        return emu.getregister('PC')
    end
    function get_stack_pointer()
        return emu.getregister('S')
    end
    function register_callback_execute(function_)
        event.onmemoryexecuteany(function_)
    end
    function script_suffix()
        event.onexit(function() io.close(log_file) end)
        while true do
            emu.frameadvance()
        end
    end
else -- mesen (assumed)
    function read_byte(address)
        return emu.read(address, emu.memType.cpuDebug)
    end
    function read_word(address)
        return emu.readWord(address, emu.memType.cpuDebug)
    end
    function get_cycles()
        return emu.getState().cpu.cycleCount
    end
    function get_program_counter()
        return emu.getState().cpu.pc
    end
    function get_stack_pointer()
        return emu.getState().cpu.sp
    end
    print = emu.log
    function register_callback_execute(function_)
        emu.addMemoryCallback(function_, emu.memCallbackType.cpuExec, 0x0, 0xffff)
    end
    function script_suffix() end
end

log_file = io.open('call_log.txt', 'w')

returns = {}
last_cycles = 0

register_callback_execute(function()
    pc = get_program_counter()
    opcode = read_byte(pc)
    new_cycles = get_cycles() - last_cycles

    if opcode == 0x20 then -- JSR
        operand = read_word(pc + 1)

        log_file:write(string.format('jsr to %04x (pc: %04x, cycles: +%d)\n', operand, pc, new_cycles))
        returns[pc + 2] = operand
        last_cycles = get_cycles()
    elseif opcode == 0x60 then -- RTS
        return_address = read_word(0x100 + get_stack_pointer() + 1)

        return_string = returns[return_address] and
                            string.format('%04x', returns[return_address]) or
                            'unknown'

        log_file:write(string.format('rts from %s (pc: %04x, cycles: +%d)\n', return_string, pc, new_cycles))
        returns[return_address] = nil
        last_cycles = get_cycles()
    end
end)

script_suffix()
