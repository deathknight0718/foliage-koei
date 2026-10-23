#pragma endian little

struct Head {
	u32 id;
	u32 size;
};

// 初始头：id = 0x33，包含 12 条本地化名称记录。
struct Record00 {
	char16 f1[3];
	char16 f2[7];
};

struct Record01 { u8 f1[0x1F]; };
struct Record02 { u8 f1[0x35]; };
struct Record03 { u8 f1[0x28]; };
struct Record04 { u8 f1[0x02]; };

// id = 4：f2 个三字节元素。
struct Record05 {
	u32 f1;
	u32 f2;
	u8 f3[f2 * 3];
};

struct Record06 { u8 f1[0x41]; };
struct Record07 {};
struct Record08 { u8 f1[0x33]; };
struct Record09 { u8 f1[0x34]; };
struct Record10 {};
struct Record11 { u8 f1[0x92]; };
struct Record12 { u8 f1[0x58]; };
struct Record13 { u8 f1[0x16]; };
struct Record14 { u8 f1[0x7F]; };
struct Record15 { u8 f1[0xA4]; };
struct Record16 { u8 f1[0x2E]; };
struct Record17 { u8 f1[0x66]; };
struct Record18 { u8 f1[0x94]; };
struct Record19 { u8 f1[0x15]; };
struct Record20 { u8 f1[0x0D]; };
struct Record21 {};
struct Record22 { u8 f1[0xA9]; };
struct Record23 {};
struct Record24 {};
struct Record25 {};
struct Record26 { u8 f1[0x86]; };
struct Record27 {};
struct Record28 { u8 f1[0x0B]; };
struct Record29 { u8 f1[0x1A]; };
struct Record30 {};
struct Record31 {};
struct Record32 { u8 f1[0x0E]; };
struct Record33 {};
struct Record34 {};

Head head00 @ $;
Record00 records00[head00.size] @ $;
Head head01 @ $;
Record01 records01[head01.size] @ $;
Head head02 @ $;
Record02 records02[head02.size] @ $;
Head head03 @ $;
Record03 records03[head03.size] @ $;
Head head04 @ $;
Record04 records04[head04.size] @ $;
Head head05 @ $;
Record05 record05 @ $;
Head head06 @ $;
Record06 records06[head06.size] @ $;
Head head07 @ $;
Record07 records07[head07.size] @ $;
Head head08 @ $;
Record08 records08[head08.size] @ $;
Head head09 @ $;
Record09 records09[head09.size] @ $;
Head head10 @ $;
Record10 records10[head10.size] @ $;
Head head11 @ $;
Record11 records11[head11.size] @ $;
Head head12 @ $;
Record12 records12[head12.size] @ $;
Head head13 @ $;
Record13 records13[head13.size] @ $;
Head head14 @ $;
Record14 records14[head14.size] @ $;
Head head15 @ $;
Record15 records15[head15.size] @ $;
Head head16 @ $;
Record16 records16[head16.size] @ $;
Head head17 @ $;
Record17 records17[head17.size] @ $;
Head head18 @ $;
Record18 records18[head18.size] @ $;
Head head19 @ $;
Record19 records19[head19.size] @ $;
Head head20 @ $;
Record20 records20[head20.size] @ $;
Head head21 @ $;
Record21 records21[head21.size] @ $;
Head head22 @ $;
Record22 records22[head22.size] @ $;
Head head23 @ $;
Record23 records23[head23.size] @ $;
Head head24 @ $;
Record24 records24[head24.size] @ $;
Head head25 @ $;
Record25 records25[head25.size] @ $;
Head head26 @ $;
Record26 records26[head26.size] @ $;
Head head27 @ $;
Record27 records27[head27.size] @ $;
Head head28 @ $;
Record28 records28[head28.size] @ $;
Head head29 @ $;
Record29 records29[head29.size] @ $;
Head head30 @ $;
Record30 records30[head30.size] @ $;
Head head31 @ $;
Record31 records31[head31.size] @ $;
Head head32 @ $;
Record32 records32[head32.size] @ $;
Head head33 @ $;
Record33 records33[head33.size] @ $;
Head head34 @ $;
Record34 records34[head34.size] @ $;
