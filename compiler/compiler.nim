import os
import strformat

const bufferSize = 512

proc compile(input: File, output: File): void =
    var rBuffer: array[bufferSize, uint8]
    var wBuffer: array[bufferSize, uint8]
    var bytesToWrite = 0
    var currentByte:uint8 = 0
    var currentBytePosition = 0

    while true:
        let bytesRead = input.readBytes(rBuffer, 0, bufferSize)

        for i in 0..<bytesRead:
            # echo ""
            let readByte = rBuffer[i]
            let bit:uint8 = 1'u8 shl (7 - currentBytePosition)
            # echo bit
            if readByte == uint8('0'):
                # echo "low"
                currentByte = currentByte and (not bit)
            elif readByte == uint8('1'):
                # echo "high"
                currentByte = currentByte or bit
            else:
                # echo "none"
                continue

            # echo &"currentByte: {currentByte}"
            currentBytePosition += 1
        
            if currentBytePosition > 7:
                # echo "NEW BYTE"
                wBuffer[bytesToWrite] = currentByte
                bytesToWrite += 1
                currentByte = 0
                currentBytePosition = 0

            if bytesToWrite == bufferSize:
                # Handle write fail
                echo &"DUMPING WRITE. BYTES TO WRITE: {bytesToWrite}"
                discard output.writeBytes(wBuffer, 0, bytesToWrite)
                bytesToWrite = 0
            
        # We've reached the end of the file
        if bytesRead < bufferSize:
            echo &"FINAL WRITE. BYTES TO WRITE: {bytesToWrite}"
            discard output.writeBytes(wBuffer, 0, bytesToWrite)
            break

proc main(): void =
    let params = os.commandLineParams()

    if params.len == 0:
        echo "Must pass in a file path"
        quit(1)

    for filename in params:
        if os.existsFile(filename):
            echo &"Processing {filename}"
            let input = open(filename, fmRead)
            defer: input.close()

            let output = open(filename.changeFileExt("bin"), fmWrite)
            defer: input.close()

            input.compile(output)

main()
