# baby-nes

NES Emulator written in Rust.

## ToDo

- [x] Cartridge
- [ ] CPU
- [ ] APU
- [x] RAM
- [ ] BUS
- [ ] PAD
- [ ] Main loop
- [ ] CLI Logic
- [ ] Debugger


## Milestone

- [ ] Extract sprite image.
- [ ] Hello world
- [ ] Debugger
- [ ] CPU test rom
- [ ] Gikoneko sample
- [ ] Super Mario Bros

## Memo

- [x] パターンテーブルからSpriteを作るところ用意
  - Spriteの責務？パターンテーブルの責務？
  - 多分PPU？
- [x] ここで一旦テストを通す
- [x] PPUのrunを作成
- [x] CPUのcycle考慮
- [x] Frameへの書き込みとRenderを追加
- [x] ここでHelloWorldが動くはず
- [ ] 全体リファクタとかテストの追加
  - [ ] 画面レンダリングを別ファイルへ（NES_COLORSも移動）
  - [ ] CpuBusのテスト
  - [ ] Cpuの各命令テスト
  - [ ] CpuRegisterテスト
  - [ ] PaletteRam見直し
  - [ ] Palette見直し
  - [ ] Ppuのbuild_tile見直し
  - [ ] Ppuのwrite副作用テスト
- [ ] CPUとにかく実装
  - 全部実装したらテストROM通す
