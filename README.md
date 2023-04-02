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
- [ ] PPUのrunを作成
- [ ] CPUのcycle考慮
- [ ] Frameへの書き込みとRenderを追加
- [ ] ここでHelloWorldが動くはず
- [ ] 全体リファクタ
  - 特に急エミュレーターからコピペしたとこ
  - Ppu RegisterのR/WとPpuBusのR/Wの関係性とか
- [ ] CPUとにかく実装
  - 全部実装したらテストROM通す
