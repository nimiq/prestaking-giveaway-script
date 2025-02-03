# Nimiq Pre-staking Giveaway Script

The pre-staking giveway prizes are calculated in three rounds. Each round's random-number-generators (RNGs) are seeded by an election block's number and hash.

Each address can only win once, and winning addresses are removed from the list of eligible addresses for the following rounds.

This script loads the list of eligible prestaker addresses and their accumulated points for each round and selects a winner for each of the round's prizes.

The list of prizes is shuffled with an RNG seeded by the block number, the winners are selected with an RNG seeded by the block hash, using the same `DiscreteDistribution` mechanism that is used to distribute validator slots on the PoS chain.

To run, pass the round number (1-3), the block number and the block hash (in this order) as arguments to the script, for example:

```bash
# Run the script for round 1, with block number 7557700 and hash 6bede69541d2cf748f023866d533deaf3819212e40e2af51012c6e1e825f6d95
cargo run -- 1 7557700 6bede69541d2cf748f023866d533deaf3819212e40e2af51012c6e1e825f6d95
```

Following the production of each round's election block, this repository will be updated with the round's winners and the list of prestakers eligible for the next round.

## Round 1

```
Round: 1
Block number: 9979200
Block hash: e26ec8e00ec3cf66ae6825e2048b15d01a72bf483bc8e2c9d568d7d271cb8a1f

Eligible prestakers: 1054

Address, Prize
NQ57 FXG6 8HSA 4LYG 0SAM HAK9 T1CV UMAK CP7V, 500k NIM
NQ03 QG8J 5UXF SU01 7M7B 72J6 E066 H3KL ED31, 1.5M NIM
NQ91 37GS R7PU PJBR M9RS 27YU HASS GEJH 14QG, 500k NIM
NQ58 7DDM 78Y9 9Q86 T1T5 Y49N 49JX HT15 N70A, 500k NIM
NQ33 1BXG CP11 UDQG 4XGQ 6102 DNTA KA7N 236C, 500k NIM
NQ04 37KM FKR0 B60A U60Q A8SC V6R9 B20P D3MS, 1.5M NIM
NQ51 7J0X 40Y0 YSR1 L3KM J0XP 512B R0AP JUH0, 3M NIM
NQ23 P7EK LC1D PEC8 3CA4 2QFQ ABEM 15G9 9H8K, 500k NIM
NQ84 1PR1 5LAS C29N NGA1 1MBU 6CJA L24H F2HX, 1.5M NIM
NQ93 N65R S65G YQ7H B1P3 F0D8 VYEJ BS4K UJBF, 500k NIM
NQ15 992L FLGL XRU7 RL9P FPKB KARY 92BB SCQS, 500k NIM
NQ31 82JN 74TL SEJ7 QYYP C7E7 U9CV A2S4 6SQ2, 1.5M NIM
NQ93 6Y98 FJV7 SP0L 41D1 NNE7 U9TL U8BS 4R15, 500k NIM
NQ63 29CL 7KFY 6UL0 45DP 19T1 GB9Y A81B 1JMY, 500k NIM
NQ76 PUL1 FLGR SBD8 8RKT CF3V FCHX XT62 UJ4R, 1.5M NIM
NQ21 0RAA 5A3C 7Q3A 6R8M 6JRC SUE4 F4RN CG8R, 500k NIM
NQ75 BD1F XSNX 451E RQNH CNY4 ADA3 96K5 YVJB, 500k NIM
NQ88 11GJ 1X6F 9RLE 71UU 0X7E 1HK6 H6GA A4QK, 500k NIM
NQ31 YA4V RKS3 U4K1 S8HY 0S8F L7X5 4QCH 7CEV, 500k NIM
NQ52 E5KA QHPE JRGQ YA4U TXEB PAPS QXK8 XRJ1, 1.5M NIM
NQ19 X3AE Q1MD PJVJ BL7V N1BC M7SR 0QUL V9RV, 500k NIM
NQ25 H49H 82SH XSPE 0VHH 1S54 J4N7 EX4E JJ4L, 3M NIM
NQ95 LN2F BC5Q MPVV F11G R5MH M740 J6KT VH1C, 500k NIM
NQ43 81MN Y681 VF9R T9K4 62XX 5QRV 9393 1AK9, 1.5M NIM
NQ97 2REE 6F3B A173 8FFF 7VFM UXJ4 KRL2 PUJV, 500k NIM
NQ76 F556 PCY5 YRDJ KCY1 5BSY 0FGJ THDC 951R, 500k NIM
NQ03 25DQ F70X PU9Y D6V1 59J4 QCPG B2KV LUNF, 500k NIM
NQ39 XKD5 1QMK EGA6 QETY J1HN PFLG 3XUJ KDTE, 1.5M NIM
NQ49 G7HX AH0F 2YE5 QEDC 62UE PH7D LV9N 9SCR, 500k NIM
NQ22 3GKC N6KP 88B6 DM9L 52NX 4S55 E32D BD76, 500k NIM
NQ49 97B3 L73T R7NT VTVX KNJ7 QNNK JLPQ L99B, 3M NIM
NQ81 99U7 6M1U 6QMX NLLP F2Q8 M2YA PP6N 5NJM, 500k NIM
```

## Round 2

_Coming February 10, 2025_

## Round 3

_Coming February 17, 2025_
