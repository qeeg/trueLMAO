    dc.l System_Stack, EntryPoint
    nop
    nop
    nop
    nop

EntryPoint:
    ori.b	#$F,d0
    ori.w	#3,d0
    ori.w	#6,($FFFF8080).w
    ori.l	#$FFFFFF,d0	; 8x8 tile pointer

    or.b	d1,$4(a0)
    or.w	d1,d0	; does nothing now
    or.l	d1,d0	; does nothing now
    or.w	PortA_Ok(pc,d7.w),d5	; only keep X lower bits


    tst.l	($A10008).l	; test ports A and B control
    beq.w       PortA_Ok
    bne.w	EntryPoint	; If so, branch.
    tst.w	($A1000C).l	; test ports A and B control
    bne.w	EntryPoint	; If so, branch.
    lea	        PortA_Ok(pc),a5
    movem.w	(a5)+,d5-d7
    movem.l	(a5)+,a0-a4
PortA_Ok:
    nop
    nop
    nop
    nop
skip:
System_Stack:

    bra.s *-$0
    bra.w EntryPoint
    bra.s *-$4
    bra.w EntryPoint
    bra.s *-$A
    bra.s *-$14

asd:

    ext.w       d5
    ext.l       d3
    movem.l	($A000).l,d0-d3/d5-d6
    movem.l	($A000).l,d0
    movem.l	($A000).l,d0-d7
    movem.w	(a5)+,d5-d7
    movem.l	(a5)+,a0-a4
    movem.l	d0-d3,-(sp)
    movem.l	d0-d7,-(sp)
    movem.l	-4(a3, a2.l),d0-a7
    movem.l	d0-a6,-(sp)
    movem.w	(a5)+,a0
    movem.l	d0-a1/a3-a5,-(sp)
    movem.l	(sp)+,d0-a1/a3-a5
    movem.l	d0-d7,($1000).w
    moveq       #$A, d5
    moveq       #$FF, d0
    and.b	d1,$4(a0)
    and.w	d1,d0	; does nothing now
    and.l	d1,d0	; does nothing now
    and.w	PortA_Ok(pc,d7.w),d5	; only keep X lower bits
    andi.b	#$F,d0
    andi.w	#3,d0
    andi.w	#6,($FFFF8080).w
    andi.l	#$FFFFFF,d0	; 8x8 tile pointer
    ; add.l       (pc, a2.l), d6
    cmp.b       ($F0000000).l, d0
    jmp         EntryPoint
    jsr         EntryPoint
    bsr.w	EntryPoint
    sub.w       (a0), d0
    neg.w        $4(a0)
    move.l	-4(a3, a2.l),d0
    move.l	d0,-4(a3, a2.l)
    clr.b       d0
    move.l      -4(a3, a2.l), 4(a3, a2.l)
    move.l      #3, 4(a3, a2.l)
    move.l      4(a3, a2.l), d6
    move.b      ($A000).l, d0
    move.l      #3, d0
    tst.b       ($A10008).l
    move.l	-4(a3, a2.l),d0
    move.l	d0,-4(a3, a2.l)
    clr.b       d0
    move.l      -4(a3, a2.l), 4(a3, a2.l)
    move.l      #3, 4(a3, a2.l)
    move.l      4(a3, a2.l), d6
    move.b      ($A000).l, d0
    move.l      #3, d0
    tst.b       ($A10008).l
    bra.w       PortA_Ok
    tst.l	(a4, a7.w)
    tst.l	4(a3, a2.l)
    tst.l	4(a0)
    move.l	d0,-4(a3, a2.l)
    tst.l	4(a3, a2.l)
    tst.l	-4(a0)

    bhi.w   EntryPoint
    bls.w   EntryPoint
    bcc.w   EntryPoint
    bcs.w   EntryPoint
    blo.w   EntryPoint
    bvc.w   EntryPoint
    bvs.w   EntryPoint
    bpl.w   EntryPoint
    bmi.w   EntryPoint
    bge.w   EntryPoint
    blt.w   EntryPoint
    bgt.w   EntryPoint
    ble.w   EntryPoint
    move.w	-4(pc),d0
    move.l	-4(a3),d0

    move.l	-4(a3, a2.l),d0
    rts

    nop
    rts
