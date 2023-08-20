# Abstract GameShark Language (AGL)

## Related Projects

[AGL Windows Editor](https://github.com/retro-git/AGLEditor/)             |  [AGL MacOS/iPadOS Editor](https://github.com/retro-git/AGLEditorSwift/)
:-------------------------:|:-------------------------:
![235808032-169061ce-b647-45da-8e6a-e76f3b754cd0](https://github.com/retro-git/agl/assets/43223593/8559f2db-ce35-4a63-a3cf-60992634bb82)  |  ![ezgif-3-4177e9e1d8](https://github.com/retro-git/agl/assets/43223593/2f09d07b-1375-401d-84f9-eba8e717cbf7)








## About
AGL is a domain-specific language that compiles to the GameShark code format. GameShark codes are notoriously archaic to both write and read - even moreso than assembly due to the lack of mnemonics and variables. AGL aims to remedy that problem with a much more familiar, high-level syntax.

Note that currently, the language only supports compilation to PSX and N64 formats. Also, this project is in an early state and hasn't been thoroughly tested yet. Some feature ideas are below - feel free to PR, give feedback/feature ideas, and report bugs.

## Example usage
- Let's say we want to create a code for Spyro 2 that increases lives to 99 and gems to 10,000 when the L2 + R2 + Triangle buttons are pressed.
- The button states can be found at address 0x800683A0, life count at 0x8006712C, and gem count at 0x800670CC.
- Therefore, we can write the following AGL:

```
INPUT_STATES = 0x683a0;
GEMS = 0x670CC;
LIVES = 0x6712C;
L2_R2_TRIANGLE = 0x13;

if(16, INPUT_STATES == L2_R2_TRIANGLE) {
    write(16, GEMS, 10000);
    write(8, LIVES, 99);
}
```
- If we save this code to the file `spyro2_gems_lives.agl`, we can then compile it with `agl spyro2_gems_lives.agl --mode psx`.
- This outputs the following raw GameShark to `spyro2_gems_lives.gs`:
```
D00683a0 0013
800670cc 2710
D00683a0 0013
3006712c 0063
```
- Entering this compiled GameShark into an emulator (or any other way to run GameShark), we can see that the code works as expected.

## Future development ideas
* Better error messages when compilation fails.
* Frontend website to try out the language without the CLI tool, with built-in examples.
* Support for more platforms (PS2, Dreamcast).
* Support encoding variables in binary.
* Syntax highlighting.
