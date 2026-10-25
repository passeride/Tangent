# Tangent – kveldtest med Yamaha P-45 (ca. 10 minutter)

Last ned **Windows x64** fra siste vellykkede bygg av `main` i
[GitHub Actions](https://github.com/passeride/Tangent/actions). Pakk ut begge ZIP-lagene.
Koble P-45 til USB, slå det på og lukk andre musikkprogrammer.
Kjør gjerne `Test-Tangent.ps1` med PowerShell, eller start `Tangent.exe` direkte.

1. **Oppstart:** Eget Tangent-vindu åpner, og Play/Library/Train/FreeForm/Settings virker.
2. **Bibliotek:** Velg Library → Import files, marker to MIDI/MusicXML/MXL-filer med Ctrl og importer. Begge skal vises. Importer én på nytt: antallet skal være uendret.
3. **Søk:** Edit → endre tittel og komponist → Save. Søk etter deler av begge navnene, også med andre store/små bokstaver. Prøv Clear, sortering og formatfilter; de skal gi forventede treff.
4. **Omstart:** Lukk og åpne Tangent. Stykkene og endringene skal være bevart. Åpne et stykke: riktig tittel og noter vises, avspillingen starter pauset. Space starter/stopper, V bytter visning.
5. **P-45 / PC-lyd:** Gå til FreeForm. Kontroller at lyden kommer fra PC-en, ikke bare pianoets egne høyttalere. Spill enkelttoner, en akkord og raske gjentakelser. Riktige tangenter lyser, slipper når du slipper, og lyden følger uten sjenerende forsinkelse.
6. **MIDI-reskanning:** Hold en tone og trykk Ctrl+M. Slipp og spill igjen. Ingen tone skal henge, og pianoet skal fortsatt fungere. Koble deretter USB fra/til og prøv Ctrl+M igjen.
7. **Lyd og belastning:** Prøv svakere/sterkere anslag og volum med opp/ned-pil. Spill et tettere stykke og bytt visning: noter, lyd og grensesnitt skal henge med uten tydelig hakking.
8. **MIDI-utgang:** Settings → Audio & MIDI → Rescan / connect MIDI → åpne MIDI output-nedtrekksmenyen og velg P-45/Digital Piano. Spill et stykke: lyden skal komme fra pianoet. I FreeForm skal egne anslag ikke gi doble toner. Pause/spoling/bytte til Computer audio og lukking skal stoppe appens toner. La Local Control være på; bruk pianoets eget volum.
9. **Lagring og gjenoppretting:** Avslutt og start Tangent på nytt. MIDI-utgangen skal velges automatisk når bare én tilkoblet port har samme navn. Koble fra pianoet, start appen og sjekk at datamaskinlyd brukes uten at den lagrede preferansen forsvinner. Koble til og kjør Rescan / connect MIDI for å gjenopprette. Prøv å velge Computer audio i nedtrekksmenyen, start på nytt og bekreft at dette er valgt.
10. **Noteøving og farger:** Train → Notes: prøv Treble/Bass/Both og nivåene. Ingen bokstavfasit før Show hint. Riktig note blir stående til du slipper; feil oktav går ikke videre. I Play er sangens forventede taster bare glød, mens egne trykk er solide, også med Sparkles av.
11. **Sletting:** Library → Remove → Confirm på ett stykke. Bare dette stykket skal forsvinne, også etter omstart. Settings → Export database skal lage en `.sqlite`-fil.

Noter **OK/FEIL**, hvilket steg, og hva du så/hørte. Ved forsinkelse: si gjerne om
du brukte PC-høyttalere, kabel eller Bluetooth. Sustainpedal vurderes ikke i denne
testen; støtte for pedal fra fysisk MIDI er fortsatt et eget arbeidspunkt.

PowerShell-starteren kjører først en enkel programtest, åpner Tangent og ber om
korte testresultater når du lukker det. Rapporten lagres ved siden av programmet.

## Practice reference and visual distance

- Train → Notes: Hear target sounds briefly without completing the exercise.
- Play a wrong octave: the coloured note shows the real pitch and ±12 semitones.
- Press the right note: target stays until full key release. Repeat with bass clef.
- Hear target, then play/pause/leave immediately: no hanging note or hardware echo.
- Effects → Punchy should feel stronger. Reduced motion disables bursts; played
  notes and expected-key glow remain. Restart and check both settings persist.
