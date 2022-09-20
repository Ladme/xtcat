// Released under MIT License.
// Copyright (c) 2022 Ladislav Bartos

/* Simple & fast concatenation of xtc files. Always skips the first frame in subsequent files! */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* Reads a 32 bit number from xdr file at target position (in bytes from the start of the file). */
uint32_t read_xdr_int(FILE *file, size_t pos)
{
    fseek(file, pos, SEEK_SET);

    unsigned char bytes[4] = {0};
    fread(bytes, 1, 4, file);

    uint32_t number = bytes[0] << 24 | bytes[1] << 16 | bytes[2] << 8 | bytes[3];

    return number;
}

int main(int argc, char **argv)
{
    if (argc < 3) {
        printf("Usage: %s -f XTC_FILE1 XTC_FILE2 ... -o OUTPUT_XTC\n", argv[0]);
        return 1;
    }

    // parse arguments
    char *output_filename = NULL;
    char *input_filenames[100] = {0};
    int block_input = 0;
    for (int i = 1; i < argc; ++i) {
        if (!strcmp(argv[i], "-o")) {
            ++i;
            output_filename = argv[i];
            continue;
        }

        if (!strcmp(argv[i], "-f")) {
            block_input = 1;
            continue;
        }

        if (block_input) {
            input_filenames[block_input - 1] = argv[i];
            ++block_input;
        }
    }

    // output info
    printf("Concatenating %d files: ", block_input - 1);
    for (int i = 0; i < block_input - 1; ++i) {
        printf("%s ", input_filenames[i]);
    }
    printf("\nOutput file: %s\n\n", output_filename);

    // open output file
    FILE *output = fopen(output_filename, "wb");
    if (output == NULL) return 1;

    for (int i = 0; i < block_input - 1; ++i) {
        printf("Concatenating file %s...\n", input_filenames[i]);

        // open the xtc file
        FILE *input = fopen(input_filenames[i], "rb");
        if (input == NULL) {
            fclose(output);
            fprintf(stderr, "Could not open file %s\n", input_filenames[i]);
            return 1;
        }

        uint32_t start = 0;
        if (i != 0) {
            // we have to include the size of the header (92 bytes)
            start = read_xdr_int(input, 88) + 92;
            // the size of the frame in bytes must be divisible by 4
            // therefore, we add some padding
            if (start % 4 != 0) start += 4 - (start % 4);
        }

        fseek(input, 0, SEEK_END);
        long filelen = ftell(input) - start;
        fseek(input, start, SEEK_SET);

        char *buffer = calloc(filelen, 1);
        if (buffer == NULL) {
            fclose(input);
            fclose(output);
            fprintf(stderr, "Could not allocate memory for buffer. File is too large?\n");
            return 1;
        }

        fread(buffer, filelen, 1, input);
        fclose(input);

        // write the buffer
        fwrite(buffer, filelen, 1, output);
        free(buffer);

    }

    fclose(output);
    return 0;
}