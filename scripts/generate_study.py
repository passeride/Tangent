#!/usr/bin/env python3
"""Reproduce Tangent's original two-hand MusicXML demo (CC0-1.0). No MIDI conversion."""
from pathlib import Path
from xml.etree.ElementTree import Element, SubElement, indent, tostring
root=Element('score-partwise', version='4.0')
def node(parent,tag,text=None,**attrs):
    e=SubElement(parent,tag,attrs)
    if text is not None:e.text=str(text)
    return e
node(node(root,'work'),'work-title','First light - Tangent piano study')
ident=node(root,'identification');node(ident,'creator','Tangent contributors',type='composer');node(ident,'rights','CC0 1.0 Universal — original demonstration score')
plist=node(root,'part-list');sp=node(plist,'score-part',id='P1');node(sp,'part-name','Piano')
part=node(root,'part',id='P1')
melodies=[[60,64,67,64],[62,65,69,65],[64,67,72,67],[62,65,67,71],[60,64,62,71],[72,67,64,60],[64,67,71,67],[65,69,72,69],[67,71,74,71],[69,67,65,62],[60,64,67,72],[65,64,62],[60,60],[62,65,69],[67,65,64],[60]]
basses=[48,50,48,43,43,48,40,41,43,45,43,41,48,50,43,48]
letters=['C','C','D','D','E','F','F','G','G','A','A','B']
for i,keys in enumerate(melodies,1):
    m=node(part,'measure',number=str(i))
    if i in (1,12):
        a=node(m,'attributes')
        if i==1:node(a,'divisions',4);node(node(a,'key'),'fifths',0)
        time=node(a,'time');node(time,'beats',4 if i==1 else 3);node(time,'beat-type',4)
        if i==1:
            node(a,'staves',2)
            for num,sign,line in [(1,'G',2),(2,'F',4)]:c=node(a,'clef',number=str(num));node(c,'sign',sign);node(c,'line',line)
    if i in (1,9):
        direction=node(m,'direction',placement='above');dt=node(direction,'direction-type');met=node(dt,'metronome');node(met,'beat-unit','quarter');node(met,'per-minute',96 if i==1 else 108);node(direction,'sound',tempo=str(96 if i==1 else 108))
    if i in (1,7):node(node(m,'barline',location='left'),'repeat',direction='forward')
    if i in (5,6,10,12):node(node(m,'barline',location='left'),'ending',number='1' if i in (5,10) else '2',type='start')
    total=16 if i<12 else 12
    durations=[4]*len(keys)
    if i==13:durations=[8,4]
    if i==16:durations=[12]
    for key,duration in zip(keys,durations):
        n=node(m,'note');p=node(n,'pitch');node(p,'step',letters[key%12]);node(p,'octave',key//12-1);node(n,'duration',duration);node(n,'voice',1);node(n,'type','half' if duration>=8 else 'quarter')
        if duration==12:node(n,'dot')
        node(n,'staff',1)
    node(node(m,'backup'),'duration',total)
    # Left hand: a rooted fifth, whole or dotted-half notes.
    for j,key in enumerate([basses[i-1],basses[i-1]+7]):
        n=node(m,'note')
        if j:node(n,'chord')
        p=node(n,'pitch');node(p,'step',letters[key%12]);node(p,'octave',key//12-1);node(n,'duration',total);node(n,'voice',2);node(n,'type','whole' if total==16 else 'half')
        if total==12:node(n,'dot')
        node(n,'staff',2)
    if i in (5,6,11,13,16):
        b=node(m,'barline',location='right')
        node(b,'bar-style','light-heavy' if i==16 else 'light-light')
        if i!=16:node(b,'ending',number='1' if i in (5,11) else '2',type='stop' if i in (5,11) else 'discontinue')
        if i in (5,11):node(b,'repeat',direction='backward',times='2')
indent(root)
path=Path(__file__).resolve().parents[1]/'web/piano/samples/first-light.musicxml'
path.write_bytes(b'<?xml version="1.0" encoding="UTF-8"?>\n'+tostring(root,encoding='utf-8')+b'\n')
print(path)
