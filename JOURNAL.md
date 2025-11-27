# DOSember Game Jam Development Journal

## 2025-11-22

Bootstrapped the project by copying stuff from the july game jam (tilers) and dos-rs.
Made a small creature sprite sheet with 4 types of shapes and 4 types of eyes.
Wrote some code for the intended graphics and palette handling, more to come.

Won't compile in debug mode, linker errors probably caused by the translation.

```none
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.12.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.12:(.text+0x334): undefined reference to `fma'
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.12.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.12:(.text+0x35e): undefined reference to `fma'
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.12.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.12:(.text+0xb): undefined reference to `'
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.12.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.12:(.text+0x1f): undefined reference to `'
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.12.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.12:(.text+0x26): undefined reference to `'
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.12.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.12:(.text+0x18): undefined reference to `'
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.12.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.12:(.text+0x13): undefined reference to `'
/usr/lib/gcc/i686-pc-msdosdjgpp/14.1.0/../../../../i686-pc-msdosdjgpp/bin/ld: ../libcreateac.a(compiler_builtins-a3b6094944818f15.compiler_builtins.6c4eb1b73f1bb8d5-cgu.05.rcgu.o):compiler_builtins.6c4eb1b73f1bb8d5-cgu.05:(.text+0x82): more undefined references to `' follow
collect2: error: ld returned 1 exit status
```

It does seem to work in release mode...

## 2025-11-23

Started seeing more bizarre things happening in debug mode.
Better stick to release mode by default.

Updated the nightly toolchain just in case, seems to work.

Found a simple bitmap font (in two sizes big and small),
adapted it for use here.
Implemented the text drawing routine.

Made the menu screen sort of work.
Fixed the fade-out routine.

Adjusted creature asset loading so that `CreatureAssets` owns the pixel data.

Started implementing the creature drawing routine,
though the colors are still wrong.

Also added `clear_screen` to `dos_x`.

## 2025-11-24

Implemented most of the main create-a-creature game loop.
Player can select different shapes and eyes.
Changing creature color still does not work,
and the colors are all wrong.

Added functions in `dos_x` to read from buffer
(so as to implement transparency).

## 2025-11-25

Finally fixed the creture colors.
Implemented changing mouths.

Made text also really transparent behind it
so that it can be meshed together with other texts and things.

Composed a baseline for the theme song (only bass and lead plucks).

## 2025-11-26

Added some limbs. Decided to split legs and arms.
Also extended the available shapes, mouths, and eyes.

Implemented the logic for turning a creature into a name.

Did a proof-of-concept async Adlib player in dos-rs.

## 2025-11-27

Made some adjustments to existing assets.

Implemented the Adlib music player (first revision).

Pushed code to GitHub.
