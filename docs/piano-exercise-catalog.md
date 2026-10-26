# Tangent — øvelseskatalog for piano og musikkteori

> **Status:** Produkt-/undervisningsspesifikasjon (**forslag**, ikke implementasjon). Opprettet 2026-10-26.
> **Omfang:** 83 unike øvelser i 10 kategorier. Primærtekst på norsk; engelske produktnavn er foreslåtte UI-labels.
> **Formål:** Gi designere, pedagoger og utviklere et konkret grunnlag for å bygge spillbare Train-øvelser over tid.

## Forholdet til eksisterende Tangent

Tangent har i dag Train → Read notes / Play chords, referanselyd, adaptive repetisjoner, SQLite-lagrede forsøk og passage practice i Play. Disse er **eksisterende basisfunksjoner**, ikke ferdig implementerte versjoner av øvelsene nedenfor. Se [scores/practice](practice-progress.md), [piano player](piano-player.md), [UI/UX v2](design/tangent-ui-v2.md) og [TODO](TODO.md). En ny øvelsestype merkes først «levert» når faktisk kode, tester, UI og lagring finnes.

Katalogen skal støtte pianister fra første notelesing til avansert improvisasjon, uten å kreve kjøp av lisensiert læremateriale eller kopiering fra andre produkter. **«Rule of fifths» betyr her kvintsirkelen / circle of fifths.**

## Ordliste og presisjonsregler

- **MIDI pitch**: heltall 0–127 (eksempel C4 = 60, A4 = 69 med vitenskapelig oktavnummerering). Instrument/setting kan kalle «middle C» annerledes; bruk konsekvent intern pitch.
- **Stavemåte kontra lyd**: C♯ og D♭ kan gi samme MIDI pitch, men er ikke identiske svar i en notasjonsteorioppgave. Skill derfor `Pitch`, `SpelledPitch`, `KeySignature` og `Interval` i den pedagogiske domenemodellen.
- **Norske toner**: bruk internasjonal B som standard i engelskspråklig UI; norsk **H** = internasjonal B (pitchklasse 11) og norsk **B** = internasjonal B♭ (pitchklasse 10). Dette dokumentet skriver konsekvent **H** og **B♭** for å unngå tvetydighet; visningsnavn skal styres av språkprofil.
- **Akkorder**: rot/akkordkvalitet er ikke det samme som bass. Fri omvending skal godta korrekte pitchklasser; eksplisitt omvendingsoppgave kontrollerer også laveste tone.
- **Relativ moll**: samme faste fortegn, forskjellig tonika (C-dur / A-moll). **Parallell moll** på engelsk *parallel minor* har samme tonika (C-dur / C-moll); terminologi må forklares lokalt.
- **Melodisk moll**: klassisk øveform har hevet 6./7. trinn oppover og naturlig moll nedover; jazz-melodisk moll kan være lik i begge retninger og krever eget valg.
- **Innspilling og hender**: et MIDI keyboard sender normalt ikke fysisk fingernummer eller sikker håndidentitet. Bruk eksplisitt part/staff, demonstrert bruker-mapping eller valgt register, og vis usikkerhet.
- **Klokke og timing**: MIDI-sampling, inputlatens, lydreferansens avspillingstid og render-time må samordnes før rytmepoeng kan gis. Ikke bruk dagens pitch-coverage som om den var presis timing-score.
- **Pedal og note-off**: CC64 påvirker klingende hold, mens tangentens note-off fremdeles er relevant for artikulasjon. Manglende controller skal gi «ikke vurderbart».
- **Komfort**: repetitiv øving skal ha pauser og moderat økning i tempo. Det finnes ingen faglig forsvarlig automatisk diagnose av spenning i håndledd fra MIDI.

## Slik tolkes hver øvelse

Hver oppføring angir **nivå (0–4)**, **læringsmål/teori**, **spillflyt**, **konkret eksempel**, **kontrollerbar vurdering** og **progresjon**. Nivåene er en foreslått intern vanskelighetsakse, ikke direkte lik ABRSM/RCM/Trinity grader. For alle øvelser kan Learn vise fasit og høre referanse; Practice ha hints og gjentakelser; Challenge bruke begrenset hjelp; Mastery skal måle uassistert mestring med nye/varierte stimuli. Evt. score i kreativ improvisasjon er **beskrivende**, ikke et objektivt riktig/feil-stempel.

### Foreslått nivåakse
| Nivå | Innhold |
|---|---|
| 0 – Introduksjon | Tone/tangent, tre–fem toner, grunnpuls og én hånd. |
| 1 – Grunnleggende | Durskalaer, treklanger, begge hender, enkel rytme. |
| 2 – Mellomnivå | Moll, omvendinger, kvintsirkel, enkel gehørtrening og harmonikk. |
| 3 – Viderekommen | Flere oktaver, transponering, funksjonell harmoni, polyrytmiske forløpere. |
| 4 – Avansert | Modulasjon, jazzharmonikk, kompleks polyrytme og musikalske helheter. |

## Katalog — 83 øvelser

## 1. Notelesing og orientering (N01–N08)

Målet er rask og selvstendig kobling mellom нотasjon, klingende tone og tangent. Begynn med få noter, utvid register/fortegn og fjern støtte gradvis.

### N01 — Note Hunter

- **Nivå:** 0–2. **Læringsmål og teori:** Knytte notebilde i G-/F-nøkkel til eksakt tonehøyde og oktav.
- **Spillflyt:** Vis én tydelig note; spilleren trykker tangenten. Vis treff/avstand på notelinjene etter svaret, ikke avslør tonen på forhånd.
- **Eksempeloppgave:** Vis G4 i G-nøkkel; velg G4, ikke en G i en annen oktav.
- **Målbar evaluering / begrensninger:** Sjekk eksakt MIDI-pitch; tell første forsøk, feilanslag og eventuelle hint. Ingen krav til rytme.
- **Progresjon:** Tre–fem noter rundt C4 → begge nøkler → hjelpelinjer, flere oktaver og blandede kort.

### N02 — Keyboard Explorer

- **Nivå:** 0–1. **Læringsmål og teori:** Forstå mønsteret av svarte/hvite tangenter, tonenavn og oktavnummer.
- **Spillflyt:** Marker en tilfeldig tangent på skjermpianoet; brukeren velger tonenavn eller spiller samme tone i annet register.
- **Eksempeloppgave:** Marker C#4; velg C♯4/D♭4 etter valgt notasjon, eller finn C#5 etter instruksjon.
- **Målbar evaluering / begrensninger:** Evaluer pitchklasse og oktav i oppgaver som krever det; enharmoniske navn må tolkes kontekstuelt.
- **Progresjon:** Start med C/D/E, så alle hvite, deretter svarte tangenter og registerhopp.

### N03 — Clef Master

- **Nivå:** 0–2. **Læringsmål og teori:** Lese både diskant-/G-nøkkel og bass-/F-nøkkel uten møysommelig telling.
- **Spillflyt:** Veksle mellom nøklene i korte oppdrag; gi ledetråder som linjenoter/mellomrom ved innlæring.
- **Eksempeloppgave:** Noten A3 i bassnøkkel vises etter en tilsvarende oppgave i G-nøkkel.
- **Målbar evaluering / begrensninger:** Evaluer skrevet note som eksakt MIDI-pitch; spor svartid og feil etter nøkkel separat.
- **Progresjon:** Én nøkkel → begge blandet → hyppige nøkkelbytter og utvidede hjelpelinjer.

### N04 — Octave Jumper

- **Nivå:** 0–2. **Læringsmål og teori:** Kjenne igjen oktavekvivalens og orientere seg raskt over hele keyboardet.
- **Spillflyt:** Spill en referansetone og be om samme pitchklasse nøyaktig én eller to oktaver høyere/lavere.
- **Eksempeloppgave:** Fra C4: spill C5, deretter C3 uten skjermhint.
- **Målbar evaluering / begrensninger:** Sjekk intervall på ±12/±24 halvtoner; ikke godta annen oktav ved eksakt krav.
- **Progresjon:** Synlig måltangent → blindt registerhopp → vekslende sprang, flere oktaver og håndbytter.

### N05 — Accidental Challenge

- **Nivå:** 1–3. **Læringsmål og teori:** Skille kryss, b, oppløsningstegn og faste/tilfeldige fortegn i korrekt takt-kontekst.
- **Spillflyt:** Presenter kort notefrase med toneartsfortegn og eventuelle lokale alterasjoner; spilleren leser før fasit vises.
- **Eksempeloppgave:** I G-dur: F♯ er fast, men et F♮ i samme takt opphever det på gitt linje/oktav.
- **Målbar evaluering / begrensninger:** Sammenlign faktisk klingende MIDI-pitch med riktig notekontekst; ikke reduser alt til en enkel svart/hvit-regel.
- **Progresjon:** Isolerte fortegn → toneartsfortegn → endringer innen takt og gjennom taktstrek.

### N06 — Interval Reading

- **Nivå:** 1–3. **Læringsmål og teori:** Lese melodisk form gjennom trinn, sprang og diatoniske intervaller.
- **Spillflyt:** Vis startnote og påfølgende notebilder; oppfordre til å lese intervallstørrelser i stedet for navn hver gang.
- **Eksempeloppgave:** C4 → E4 → D4 → G4; marker sekund/ters/kvart etter hvert svar.
- **Målbar evaluering / begrensninger:** Mål eksakt pitch i sekvens og egen teorirespons for navngitt intervall; behold stavemåten.
- **Progresjon:** Sekund/ters → kvart/kvint/oktav → fortegnsendrede og sammensatte intervaller.

### N07 — Hidden Labels

- **Nivå:** 0–3. **Læringsmål og teori:** Automatisere gjenkjenning uten permanent tangentglød, bokstavnavn eller fargekoder.
- **Spillflyt:** Samme notelesingsoppdrag kjøres i trinn der hjelpetekst, fingersetting og forventet tangentglød fjernes.
- **Eksempeloppgave:** Spill 12 tilfeldig valgte noter: første runde med hjelp, siste uten.
- **Målbar evaluering / begrensninger:** Sammenlign første-forsøk-presisjon i assistert og uassistert modus; vis hjelpestatus eksplisitt.
- **Progresjon:** Reduser én støtte om gangen; repeter svake noter før tidsbegrensning innføres.

### N08 — Transpose the Melody

- **Nivå:** 2–4. **Læringsmål og teori:** Forstå intervallstruktur, toneart og praktisk transponering.
- **Spillflyt:** Vis kort skrevet melodi og måltonart; spilleren spiller samme intervaller fra ny tonika.
- **Eksempeloppgave:** C–D–E–G i C-dur transponeres til D–E–F♯–A i D-dur.
- **Målbar evaluering / begrensninger:** Sjekk transponert MIDI-melodi og rytme om valgt; enharmonisk korrekt teori må ikke bedømmes ut fra MIDI alene.
- **Progresjon:** Transponer ±2 halvtoner → nærliggende tonearter → lengre fraser og modale/atonale eksempler.


## 2. Fingerteknikk og håndkoordinasjon (T01–T08)

Teknikktrening skal forbedre stabilitet, samspill og bevegelsesplanlegging uten å rangere hastighet over kontroll. MIDI registrerer tone, varighet og anslagsstyrke – ikke faktisk fingerbruk.

### T01 — Five Finger Flow

- **Nivå:** 0–2. **Læringsmål og teori:** Bygge jevn kontroll i femfingerposisjon og gjenkjenne skalatrinn 1–5.
- **Spillflyt:** Vis fem nabotoner med forslag til 1–5; spill stigende, synkende og tilbake i rolig puls.
- **Eksempeloppgave:** C–D–E–F–G–F–E–D–C på 60 BPM, høyre hånd først.
- **Målbar evaluering / begrensninger:** Evaluer rekkefølge, tidsavvik og jevnhet i note-on; noter at fingersetting bare er veiledende.
- **Progresjon:** Én hånd → begge hender → transponerbar femfingerposisjon og aksent på hvert fjerde anslag.

### T02 — Finger Patterns

- **Nivå:** 1–3. **Læringsmål og teori:** Trene fingeruavhengighet og kontroll gjennom mønstre med hopp innen posisjon.
- **Spillflyt:** Presenter nummerert mønster og tonene det tilsvarer; repeter korte serier med hvile mellom.
- **Eksempeloppgave:** Mønster 1–3–2–4–3–5 i C-posisjon blir C–E–D–F–E–G.
- **Målbar evaluering / begrensninger:** Bedøm presise toner og jevn tidsavstand; aldri påstå at MIDI har verifisert de fysiske fingrene.
- **Progresjon:** To-/tretonersgrupper → lengre asymmetriske mønstre; øk tempo bare ved komfortabel utførelse.

### T03 — Thumb Crossing

- **Nivå:** 1–3. **Læringsmål og teori:** Lære fingerskift ved skala- og arpeggiopassasjer.
- **Spillflyt:** Vis planlagt tommelpassering visuelt og en sekvens som krysser posisjonsgrensen.
- **Eksempeloppgave:** Høyre C-dur: 1–2–3–1–2–3–4–5 ved én oktav.
- **Målbar evaluering / begrensninger:** Tonefølge/timing kan måles; valgt fingersetting må eventuelt bekreftes manuelt, ikke infereres fra MIDI.
- **Progresjon:** Én hånd én oktav → to oktaver → begge hender i parallelle og motsatte bevegelser.

### T04 — Mirror Hands

- **Nivå:** 1–3. **Læringsmål og teori:** Trene bilateral koordinasjon i speilvendt bevegelse.
- **Spillflyt:** Begge hender starter på et sentralt utgangspunkt og spiller ut fra/sammen mot midten.
- **Eksempeloppgave:** Tommelen på C4/C5 med speilvendt femfingerskala utover.
- **Målbar evaluering / begrensninger:** Sammenlign korresponderende anslag i begge registre med separat toleranse for samtidighet.
- **Progresjon:** Tre toner → fem toner → synkronisert skiftende rytme og forskjellige dynamiske styrker.

### T05 — Independent Hands

- **Nivå:** 1–4. **Læringsmål og teori:** Utvikle uavhengighet mellom melodi og akkompagnement.
- **Spillflyt:** Venstre hånd holder bassnote eller enkel puls mens høyre spiller egen rytme; bytt roller.
- **Eksempeloppgave:** Venstre holder C3 i fire slag mens høyre spiller C4–E4–G4–E4 som fjerdedeler.
- **Målbar evaluering / begrensninger:** Bedøm hver stemme og samtidighetsfeil separat, og vurder note-off når sustain ikke maskerer.
- **Progresjon:** Lang bass + enkel melodi → skiftende akkompagnement → to uavhengige rytmemønstre.

### T06 — Articulation Switch

- **Nivå:** 1–4. **Læringsmål og teori:** Forstå forskjell mellom legato, staccato og tenuto i utførelse.
- **Spillflyt:** Spill samme frase med ulike artikulasjonsinstruksjoner og lytt til forskjellen.
- **Eksempeloppgave:** Fire C-durtoner: først bundet uten tydelige opphold, deretter korte staccatoanslag.
- **Målbar evaluering / begrensninger:** Bruk MIDI note-on/off og reell overlap/gap; gi toleranse og ikke bland inn pedalstyrt klanglengde uten modell.
- **Progresjon:** To kontrastformer → veksling per takt → blandet artikulasjon innen en melodisk frase.

### T07 — Ornament Challenge

- **Nivå:** 2–4. **Læringsmål og teori:** Lære triller, forslagstoner og repeterte noter kontrollert.
- **Spillflyt:** Presenter ett ornament av gangen med tydelig utbrodert eksempel og alternativ sakte avspilling.
- **Eksempeloppgave:** Trille D–E–D–E over to slag; senere kort forslagstone før hovednoten.
- **Målbar evaluering / begrensninger:** Sammenlign tolerante note-sekvenser, forventet hovednote og metrisk avslutning; tillat stilistisk variasjon.
- **Progresjon:** Enkel dobbeltnote → lange triller → ornamenter integrert i repertoar.

### T08 — Étude Missions

- **Nivå:** 1–4. **Læringsmål og teori:** Anvende teknikk i korte musikalske studier fremfor isolert repetisjon.
- **Spillflyt:** Velg en original eller rettighetsavklart etyde; marker dagens fokus, eksempelvis sprang eller artikulasjon.
- **Eksempeloppgave:** Åttetakters studie med høyrehåndssekvenser, bassostinato og tydelig avslutning.
- **Målbar evaluering / begrensninger:** Bruk oppgaverelevant score: korrekthet, puls og valgfritt dynamisk uttrykk, ikke bare antall poeng.
- **Progresjon:** Nivåtilpassede etyder med flere stemmer og skiftende tekniske fokus.


## 3. Skalaer og skalatrinn (S01–S10)

Skalalære må forankres i toneart, korrekte notenavn og mønstre av hel-/halvtoner. Generering kan bruke MIDI-pitch, men pedagogisk visning må bevare enharmonisk stavemåte.

### S01 — Build a Scale

- **Nivå:** 0–2. **Læringsmål og teori:** Forstå hvordan intervallmønstre danner skalaer.
- **Spillflyt:** Vis tomme trinn og la spilleren fylle dem på skjerm eller fysisk piano; forklar toneartsstavemåten.
- **Eksempeloppgave:** C-dur: C–D–E–F–G–A–H–C med mønster 2–2–1–2–2–2–1.
- **Målbar evaluering / begrensninger:** Sjekk hvert skalatrinn og valgte bokstavnavn; MIDI kan bare validere lydlige trinn.
- **Progresjon:** Dur → naturlig moll → pentatonikk og kirketonearter med uavhengige konstruksjonsregler.

### S02 — Pentascale Trainer

- **Nivå:** 0–2. **Læringsmål og teori:** Lære første fem skalatrinn som teknisk og harmonisk grunnlag.
- **Spillflyt:** Velg tonika og femtoneskala; brukeren spiller opp og ned med håndvalgfri støtte.
- **Eksempeloppgave:** G–A–H–C–D–C–H–A–G med start i G-dur.
- **Målbar evaluering / begrensninger:** Kontroller eksakte toner og jevn rytme; evaluer høyre/venstre separat ved forskjellige oktaver.
- **Progresjon:** C/G/F → alle 12 tonika → begge hender, rytmisk variasjon og uten guide.

### S03 — Major Scale Run

- **Nivå:** 1–4. **Læringsmål og teori:** Beherske samtlige 12 durskalaer og skalaens fingerskifter.
- **Spillflyt:** Spill 1–4 oktaver opp/ned med valgt hånd, metronom og optional fingersetting.
- **Eksempeloppgave:** D-dur: D–E–F♯–G–A–H–C♯–D og tilbake.
- **Målbar evaluering / begrensninger:** Mål sekvensdekning, feiltoner, tempo og anslagsjevnhet; ikke gi fingerkarakter uten sensor.
- **Progresjon:** Én oktav langsomt → to/fire oktaver → begge hender, blandede tonearter og økende tempo.

### S04 — Minor Scale Lab

- **Nivå:** 1–4. **Læringsmål og teori:** Skille naturlig, harmonisk og melodisk moll – inkludert retning i klassisk melodisk moll.
- **Spillflyt:** Velg molltype, hør sammenligning og spill begge retninger med opplyste endringer.
- **Eksempeloppgave:** A: naturlig A H C D E F G; harmonisk G♯; klassisk melodisk opp F♯/G♯, ned F/G.
- **Målbar evaluering / begrensninger:** Vurder riktig type og opp-/nedretning; vis forskjell fra jazz-melodisk moll som egen innstilling.
- **Progresjon:** A/E/D-moll → alle tonearter → blandede typer uten forhåndsvarsel.

### S05 — Chromatic Run

- **Nivå:** 1–3. **Læringsmål og teori:** Trene halvtonetrinn, oversikt og jevn fingerkontroll.
- **Spillflyt:** Generer kromatiske løp i angitt register; marker tonene i notasjon og keyboard.
- **Eksempeloppgave:** C4–C♯4–D4–D♯4–E4–F4–F♯4–G4 og videre.
- **Målbar evaluering / begrensninger:** Bedøm hvert halvtonetrinn, utelatte/ekstra toner og eventuelt tempo.
- **Progresjon:** Fem noter → oktav → flere oktaver, skiftende starttoner og hender.

### S06 — Blues & Pentatonic

- **Nivå:** 1–3. **Læringsmål og teori:** Lære dur-/mollpentatonikk og bluesskala som grunnlag for improvisasjon.
- **Spillflyt:** Start med å spille skalaen, og åpne deretter fri respons over en enkel akkordloop.
- **Eksempeloppgave:** C-mollpentatonisk C–E♭–F–G–B♭; C-blues legger til G♭/F♯.
- **Målbar evaluering / begrensninger:** I ren drill: eksakt tonesett; i improvisasjon: registrer skala-tilhørighet uten å straffe bevisste utenfortoner automatisk.
- **Progresjon:** Skala alene → call-and-response → enkle bluesfraser og modulasjon.

### S07 — Modes Explorer

- **Nivå:** 2–4. **Læringsmål og teori:** Forstå modale skalaer og hvilken klang hver skaper.
- **Spillflyt:** Vis tonal startnote og karakteristiske intervaller; spill og sammenlign over dronetone.
- **Eksempeloppgave:** D-dorisk: D–E–F–G–A–H–C; F-lydisk: F–G–A–H–C–D–E.
- **Målbar evaluering / begrensninger:** Valider start/tonika, riktig trinnsett og forskjellen mellom modal tonika og parallelle durtoner.
- **Progresjon:** Jonisk/eolisk → dorisk/miksolydisk → alle syv og transponering.

### S08 — Scale Variations

- **Nivå:** 2–4. **Læringsmål og teori:** Anvende skalaer i terser, sekster og rytmiske grupperinger.
- **Spillflyt:** Velg underliggende skala og intervallmønster; generer noter og lydreferanse.
- **Eksempeloppgave:** C-dur i terser: C–E, D–F, E–G ... som sekvens eller tostemmig.
- **Målbar evaluering / begrensninger:** Bedøm riktig diatonisk intervall og utførelsesmodus (etter hverandre/samtidig).
- **Progresjon:** En hånd i terser → to hender i sekster → varierte rytmer og oktaver.

### S09 — Scale Sequences

- **Nivå:** 1–4. **Læringsmål og teori:** Gjenkjenne og spille flyttbare motiv innen en skala.
- **Spillflyt:** Gjenta et mønster fra påfølgende skalatrinn, med tydelig visning av motivgrenser.
- **Eksempeloppgave:** C-dur: C D E F | D E F G | E F G A osv.
- **Målbar evaluering / begrensninger:** Sammenlign noteorden, rytme og delvis korrekthet per motiv fremfor bare total treffprosent.
- **Progresjon:** Tretone- → firetone- → blandet stigende/synkende sekvens i moll og modi.

### S10 — Scale Degree Quiz

- **Nivå:** 1–3. **Læringsmål og teori:** Raskt finne funksjonen til tonika, mediant, dominant og ledetone.
- **Spillflyt:** Oppgi toneart og skalatrinn/tall eller funksjonsnavn; spilleren spiller riktig tone.
- **Eksempeloppgave:** I G-dur: spill 7. trinn/ledetone = F♯; 5. trinn/dominant = D.
- **Målbar evaluering / begrensninger:** Vurder korrekt pitchklasse og eventuelt oktav; skill teorioppgave fra tempoøvelse.
- **Progresjon:** Trinn 1/3/5 → alle sju → harmonisk moll, modale og akkordbundne trinn.


## 4. Intervaller, akkorder og harmonikk (H01–H09)

Skille konstruksjon (hvilke toner?), identifikasjon (hva høres?) og funksjon (hvorfor passer akkorden?). Klassisk romertallsanalyse og akkordnavn skal være eksplisitte, ikke sammenblandes.

### H01 — Interval Builder

- **Nivå:** 1–3. **Læringsmål og teori:** Bygge rene, store, små, forstørrede og forminskede intervaller.
- **Spillflyt:** Gi grunntone og intervallnavn; spilleren spiller den andre tonen, melodisk eller samtidig.
- **Eksempeloppgave:** Fra C4: stor ters E4, liten ters E♭4, ren kvint G4.
- **Målbar evaluering / begrensninger:** MIDI validerer halvtonedistanse; teori-UI validerer intervallnummer og korrekt enharmonisk navn.
- **Progresjon:** Sekund/ters/kvint → alle enkle → omvendinger og sammensatte intervaller.

### H02 — Degree Finder

- **Nivå:** 1–3. **Læringsmål og teori:** Forstå skala- og akkordfunksjoner i en valgt toneart.
- **Spillflyt:** Vis tonika/akkord og be spilleren finne 1., 3., 5. eller 7. tone.
- **Eksempeloppgave:** I F-dur finner spilleren A som tredje skalatrinn; i Dm7 finner C som septim.
- **Målbar evaluering / begrensninger:** Skill skalaens grader fra akkordtoner; kontroller korrekt pitchklasse i gitt register.
- **Progresjon:** Dur → moll → altererte akkorder og grader ved transponering.

### H03 — Chord Constructor

- **Nivå:** 1–3. **Læringsmål og teori:** Lære intervallformler for treklanger.
- **Spillflyt:** Be om bestemt akkordtype og rot; spill alle tonene samtidig eller bygg visuelt.
- **Eksempeloppgave:** F♯-moll = F♯–A–C♯; C-dim = C–E♭–G♭.
- **Målbar evaluering / begrensninger:** Sjekk pitchklasse-sett og påkrevd bass/omvending; tolerer liten onsetforskjell mellom fingre.
- **Progresjon:** Dur/moll → dim/aug → tilfeldige grunntoner, oktavdublering og ingen hints.

### H04 — Inversion Master

- **Nivå:** 1–4. **Læringsmål og teori:** Forstå grunnstilling, første og andre omvending og praktiske stemmeføringer.
- **Spillflyt:** Vis akkord og ønsket omvending; pianisten spiller alle akkordtoner med riktig basstone.
- **Eksempeloppgave:** C-dur første omvending = E–G–C; andre = G–C–E.
- **Målbar evaluering / begrensninger:** Sjekk pitchklasse-sett og laveste faktisk holdte tone; tillat doblinger kun om oppgaven sier det.
- **Progresjon:** Treklanger → fireklanger → hurtig veksling mellom omvendinger.

### H05 — Seventh Chords

- **Nivå:** 2–4. **Læringsmål og teori:** Kjenne formlene for maj7, m7, dominant7, m7♭5 og dim7.
- **Spillflyt:** Spill oppgitt septimakkord og se hvilke intervalltrinn som inngår.
- **Eksempeloppgave:** G7 = G–H–D–F; Gmaj7 = G–H–D–F♯; Gm7 = G–B♭–D–F.
- **Målbar evaluering / begrensninger:** Valider fire pitchklasser; basskrav bare ved eksplisitt omvending; opplys om enharmoni.
- **Progresjon:** Én akkordfamilie → flere → inversjoner og akkordskifter i tempo.

### H06 — Diatonic Harmony

- **Nivå:** 2–4. **Læringsmål og teori:** Forstå treklangene bygget på hvert skalatrinn i én toneart.
- **Spillflyt:** Tangent viser tonearten, romertall og en grad; spill korrekt diatonisk akkord.
- **Eksempeloppgave:** I C-dur: I=C, ii=Dm, iii=Em, IV=F, V=G, vi=Am, vii°=Hdim.
- **Målbar evaluering / begrensninger:** Sjekk funksjon og pitchsett, med separat quiz for romertall vs lydlig konstruksjon.
- **Progresjon:** C-dur → alle durtonearter → naturlig/harmonisk moll og septimakkorder.

### H07 — Cadence Explorer

- **Nivå:** 2–4. **Læringsmål og teori:** Høre og spille harmonisk spenning/oppløsning gjennom kadensmønstre.
- **Spillflyt:** Spill først guidet V–I, IV–I og V–vi; deretter identifiser den hørte kadensen.
- **Eksempeloppgave:** G7 → C er autentisk dominant–tonika i C; G7 → Am gir en bedragersk vending.
- **Målbar evaluering / begrensninger:** Vurder akkordfunksjon i riktig toneart og rekkefølge; stilisert kadens må ikke forveksles med komplett fraseanalyse.
- **Progresjon:** To akkorder → flere stemmer → ukjent toneart og gehørvariant.

### H08 — Voice Leading

- **Nivå:** 2–4. **Læringsmål og teori:** Lære å flytte stemmer økonomisk mellom akkorder uten store unødvendige sprang.
- **Spillflyt:** Foreslå en progresjon og vis total halvtonereise; spilleren finner en behagelig omvending.
- **Eksempeloppgave:** C–F–G–C med nærmeste akkordtoner, ikke alltid grunnstilling.
- **Målbar evaluering / begrensninger:** Spor individuelle stemmer bare med eksplisitt oppgitt stemme-/registrermodell; ellers mål nærhet heuristisk.
- **Progresjon:** To treklanger → fireklanger → basslinje og stemmeføringsregler for flere stemmer.

### H09 — Extended Harmony

- **Nivå:** 3–4. **Læringsmål og teori:** Bygge sus2/sus4, add9, 9-, 11- og 13-akkorder og forstå akkordutvidelser.
- **Spillflyt:** Velg akkordsymbol og oppgitt voicing/utelatelser; lytt og bygg toner.
- **Eksempeloppgave:** Cadd9 = C–E–G–D; G9 kan spilles G–H–F–A med kvinten utelatt etter oppgavens regel.
- **Målbar evaluering / begrensninger:** Valider mot eksplisitt tillatt voicing, ikke ett universelt og rigid tonesett.
- **Progresjon:** Sus/add → 9 → valgte 11/13-voicinger, alterasjoner og kontekst.


## 5. Arpeggioer og brutte akkorder (A01–A07)

Skill akkordidentitet fra rekkefølgen tonene brytes i. Arpeggiooppgaver bør ha tydelig retning, register og valgfritt fingermønster.

### A01 — Broken Triads

- **Nivå:** 0–2. **Læringsmål og teori:** Bli trygg på grunntone, ters og kvint som sekvens.
- **Spillflyt:** Spill én treklang som enkelttoner opp/ned; la akkordsymbolet bli stående.
- **Eksempeloppgave:** C–E–G–E–C i rolige åttendedeler.
- **Målbar evaluering / begrensninger:** Sjekk eksplisitt rekkefølge og timing; samme akkordtoner i annen rekkefølge gir ikke fullt treff.
- **Progresjon:** Dur → moll → alle grunntoner og mindre håndhopp.

### A02 — Arpeggio Runner

- **Nivå:** 1–4. **Læringsmål og teori:** Lære flytende treklangsarpeggioer over flere oktaver.
- **Spillflyt:** Velg toneart, akkordtype, hånd og register; følg notestav og fallende noter.
- **Eksempeloppgave:** C–E–G–C5–E5–G5–C6 og tilbake.
- **Målbar evaluering / begrensninger:** Mål tonefølge, feil, timing og jevnhet uten å rangere ren hastighet isolert.
- **Progresjon:** 1 oktav → 2–4 → begge hender og varierende tonika.

### A03 — Two-Hand Arpeggios

- **Nivå:** 1–4. **Læringsmål og teori:** Koordinere arpeggiofordeling mellom hender.
- **Spillflyt:** Begynn med en hånd om gangen; deretter samtidig eller vekslende arpeggioer.
- **Eksempeloppgave:** Venstre C3–E3–G3 og høyre C4–E4–G4 i parallell puls.
- **Målbar evaluering / begrensninger:** Kontroller separate registersekvenser og synkronisering; ikke inferer hånd hvis register overlapper uten metadata.
- **Progresjon:** Separat → unisont → vekselvis → motsatt bevegelse.

### A04 — Inversion Cycle

- **Nivå:** 2–4. **Læringsmål og teori:** Beherske brutte akkorder fra alle omvendinger.
- **Spillflyt:** Spill grunnstilling, første og andre omvending som tre separate mønstre.
- **Eksempeloppgave:** C–E–G | E–G–C | G–C–E i samme område.
- **Målbar evaluering / begrensninger:** Kontroller korrekt basstone og rekkefølge; skille fra H04, der tonene spilles samtidig.
- **Progresjon:** Dur → moll → septimakkorder med fire omvendinger.

### A05 — Seventh Arpeggios

- **Nivå:** 2–4. **Læringsmål og teori:** Spille firetone-akkorder som sekvenser med stabil håndform.
- **Spillflyt:** Generer septimarpeggio i flere registre og skift omvending.
- **Eksempeloppgave:** Dm7: D–F–A–C–D5 og tilbake.
- **Målbar evaluering / begrensninger:** Valider fire forskjellige akkordtrinn og riktig sekvens, tolerer valgte dobbeltoner om eksplisitt.
- **Progresjon:** Dominant7/m7 → maj7/m7♭5 → flere oktaver og raske vekslinger.

### A06 — Dominant & Diminished

- **Nivå:** 3–4. **Læringsmål og teori:** Lære karakteristiske symmetriske/ustabile arpeggioformer.
- **Spillflyt:** Spill dominantseptim og forminsket septim hver for seg, deretter i harmonisk kontekst.
- **Eksempeloppgave:** G7: G–H–D–F; Hdim7: H–D–F–A♭.
- **Målbar evaluering / begrensninger:** Sjekk eksakt lydlig sett og kontekstuell stavemåte; distinksjon fra halvforminsket må fremgå.
- **Progresjon:** Én oktav → modulasjoner → progresjoner som leder videre til tonika.

### A07 — Flowing Patterns

- **Nivå:** 2–4. **Læringsmål og teori:** Bruke arpeggioer som sammenhengende akkompagnementsfigur.
- **Spillflyt:** Hold et kontinuerlig rytmemønster mens akkordsymbolene skifter hvert mål.
- **Eksempeloppgave:** C–G–E–G over C, så A–E–C–E over Am.
- **Målbar evaluering / begrensninger:** Mål at riktig mønster følger gjeldende akkord og puls; separat statistikk for feil i overgangen.
- **Progresjon:** To akkorder → fire akkorder → flere mønstre, inversjoner og dynamiske nyanser.


## 6. Kvintsirkelen og tonearter (K01–K08)

Kvintsirkelen knytter sammen tonearter, faste fortegn, beslektet moll, progresjoner og transponering. Illustrasjonen kan være interaktiv, men alle oppgaver må også kunne navigeres med tangentbord.

### K01 — Circle Navigator

- **Nivå:** 1–3. **Læringsmål og teori:** Forstå kvintbevegelse med/mot klokken og navngiving av tonearter.
- **Spillflyt:** Start med en toneart på en sirkelskive; be om ett, to eller flere steg i valgt retning.
- **Eksempeloppgave:** Fra C: ett trinn med klokken gir G, to trinn D; mot klokken ett trinn F.
- **Målbar evaluering / begrensninger:** Valider nøyaktig toneartsnavn inkludert enharmoniske tilfeller; hold stavemåte konsistent.
- **Progresjon:** Ett steg → flere → skjult sirkeldiagram og tilfeldig start.

### K02 — Key Signature Master

- **Nivå:** 1–3. **Læringsmål og teori:** Kjenne ordning av kryss/b og antallet for hvert toneartstegn.
- **Spillflyt:** Vis toneart og be spilleren velge/bygge korrekt fortegn på riktig staffposisjon.
- **Eksempeloppgave:** D-dur: F♯ og C♯; B♭-dur: B♭ og E♭ (i norsk tekst B♭ etter valgt system).
- **Målbar evaluering / begrensninger:** Teorivurdering må skille rekkefølge og staffplassering; MIDI-tone alene kan ikke bevise dette.
- **Progresjon:** 0–2 fortegn → 3–7 → både dur og moll uten visuell støtte.

### K03 — Relative Minor

- **Nivå:** 1–3. **Læringsmål og teori:** Koble hver durtoneart til den parallelt fortegnsdelende molltonearten (relative minor).
- **Spillflyt:** Vis durtonearten og be om mollnavn, tonika på pianoet og eventuelt skalaspill.
- **Eksempeloppgave:** C-dur ↔ A-moll; G-dur ↔ E-moll.
- **Målbar evaluering / begrensninger:** Sjekk valgt toneartsnavn og grunntone; skille *relative minor* fra *parallel minor* (samme tonika).
- **Progresjon:** C/G/F → alle tonearter → bytt retning moll→dur og legg til skalaform.

### K04 — Circle Walk

- **Nivå:** 1–3. **Læringsmål og teori:** Huske kvintrekken ved å spille grunntoner i stadig ny toneart.
- **Spillflyt:** Spill tonika én om gangen gjennom kvintsirkelen, med visuell progressjon.
- **Eksempeloppgave:** C–G–D–A–E–H–F♯–C♯ ... over et register som velges pedagogisk.
- **Målbar evaluering / begrensninger:** Kontroller pitchklasse i sekvens, ikke bestemte oktaver med mindre register er del av målet.
- **Progresjon:** Fem trinn → full sirkel → tilfeldig start og motsatt retning.

### K05 — Circle of Triads

- **Nivå:** 2–4. **Læringsmål og teori:** Automatisere akkordformene i alle tonearter.
- **Spillflyt:** Spill en dur- eller molltreklang for hvert stopp på kvintsirkelen.
- **Eksempeloppgave:** C-dur → G-dur → D-dur med akkordtonene vist bare i læremodus.
- **Målbar evaluering / begrensninger:** Sjekk samtidige pitchklasser og aktuell rot; vurder overganger separat fra akkordtreff.
- **Progresjon:** Tre tonearter → hele sirkel → inversjoner og to hender.

### K06 — Seventh Circle

- **Nivå:** 2–4. **Læringsmål og teori:** Bevege seg gjennom kvintsirkelen med septimakkorder.
- **Spillflyt:** Spill dominant7, maj7 eller m7 etter valgt oppgavetype på hvert trinn.
- **Eksempeloppgave:** C7 → F7 → B♭7 → E♭7, eller Cmaj7 → Gmaj7 → Dmaj7.
- **Målbar evaluering / begrensninger:** Kontroller firetoners sett og akkordtype, med korrekt stavet teorivisning.
- **Progresjon:** Én type → blandet type → fraser i puls og optimale voicinger.

### K07 — ii–V–I Challenge

- **Nivå:** 2–4. **Læringsmål og teori:** Lære en sentral funksjonell jazzprogresjon i alle tonearter.
- **Spillflyt:** Vis måltonart og romertall; spill ii7 → V7 → Imaj7 i et gitt antall takter.
- **Eksempeloppgave:** I C-dur: Dm7 → G7 → Cmaj7.
- **Målbar evaluering / begrensninger:** Valider akkordfunksjon, rekkefølge og valgte omvendinger hvis oppgaven krever det.
- **Progresjon:** Guidet C/F/G → alle tonearter langs sirkelen → uten symbolhjelp og med stemmeføring.

### K08 — Modulation Quest

- **Nivå:** 3–4. **Læringsmål og teori:** Forstå hvordan en progresjon kan endre opplevd tonikasenter.
- **Spillflyt:** Spill en kort frase og flytt til nært beslektet toneart, eventuelt via pivotakkord.
- **Eksempeloppgave:** C-dur-frase → G-dur-frase der F blir F♯; bekreft ny dominant–tonika.
- **Målbar evaluering / begrensninger:** Evaluer måltonartens toner og kadens; ikke påstå at tonal modulasjon er bevist av én enkelt tone.
- **Progresjon:** Nabotonarter → pivotakkorder → fjernere overganger og analysevalg.


## 7. Rytme, puls og koordinasjon (R01–R08)

Rytme skal ikke forveksles med tonehøyde. Kalibrering og latencykompensasjon trengs før presise tidsgrenser brukes i score; pauser må oppdages som fravær av anslag.

### R01 — Beat Keeper

- **Nivå:** 0–2. **Læringsmål og teori:** Internalisere en jevn puls uten melodisk belastning.
- **Spillflyt:** Metronom gir en startpuls, deretter spiller man enkelttone på slagene, eventuelt uten klikk.
- **Eksempeloppgave:** Slå C4 på 1–2–3–4 i 60 BPM over åtte takter.
- **Målbar evaluering / begrensninger:** Mål onset-avvik fra taktgrid og intervallets varians; vurder systemlatens separat.
- **Progresjon:** Med metronom → metronom faller ut → tempoendringer og stille takter.

### R02 — Rhythm Values

- **Nivå:** 0–2. **Læringsmål og teori:** Kjenne varighet av hel-, halv-, kvart-, åttende- og sekstendedelsnoter.
- **Spillflyt:** Vis én tonehøyde med rytmefigur; spill og hold passende varighet i valgt taktart.
- **Eksempeloppgave:** I 4/4: halvnote C, to kvartnoter C, fire åttendedelsnoter C.
- **Målbar evaluering / begrensninger:** Mål onset og note-off i forhold til noteverdier; pedal og taste-release må håndteres eksplisitt.
- **Progresjon:** Store noteverdier → punkteringer og blandede underdelinger.

### R03 — Rest Challenge

- **Nivå:** 1–3. **Læringsmål og teori:** Lære aktiv stillhet, pauser og presis rytmeplassering.
- **Spillflyt:** En rytmelinje inneholder toner og pauser; ingen MIDI-anslag tillates inne i reelle pausevindu.
- **Eksempeloppgave:** I 4/4: kvart C, kvartpause, to kvart C.
- **Målbar evaluering / begrensninger:** Telling av falske anslag i pauseintervall og manglende slag; tolerer release fra forrige tone.
- **Progresjon:** Kvartpauser → åttende-/sekstendedelspauser → synkoper.

### R04 — Rhythm Echo

- **Nivå:** 0–4. **Læringsmål og teori:** Høre og gjenskape rytmiske strukturer uavhengig av absolutte noter.
- **Spillflyt:** Tangent spiller et rytmemotiv, brukeren klapper via én tangent eller spiller oppgitte toner.
- **Eksempeloppgave:** Lytt til kvart–to åttendedeler–kvartpause–kvart og gjenta.
- **Målbar evaluering / begrensninger:** Sammenlign normaliserte anslagsavstander og valgfrie varigheter; ikke kreve samme tangent i klappemodus.
- **Progresjon:** To slag → to takter → synkoper, tuplett og lengre motiver.

### R05 — Syncopation Lab

- **Nivå:** 2–4. **Læringsmål og teori:** Forstå aksenter mellom grunnslag og forskjøvede anslag.
- **Spillflyt:** Vis en enkel 4/4-groove og tren å spille på «og» mellom slagene.
- **Eksempeloppgave:** Spill på 1&, 2&, 3&, 4& over metronom med slagmarkering.
- **Målbar evaluering / begrensninger:** Mål offbeat-onset; unngå å behandle korrekt synkope som «sen» onbeat.
- **Progresjon:** Enkel offbeat → bundne synkoper over slag → kombinasjon med akkorder.

### R06 — Meter Master

- **Nivå:** 1–4. **Læringsmål og teori:** Forstå forskjellen på slaggrupper og taktfølelse.
- **Spillflyt:** Bytt 2/4, 3/4, 4/4, 6/8 og senere asymmetriske taktarter; marker sterke slag.
- **Eksempeloppgave:** Sammenlign 3/4 (tre kvartslag) med 6/8 (to grupper à tre åttendeler).
- **Målbar evaluering / begrensninger:** Bedøm aksentplassering, grupper og noteonsets; ikke likestill 6/8 mekanisk med 3/4.
- **Progresjon:** Enkle taktarter → sammensatte → 5/8 og 7/8 med tydelig gruppering.

### R07 — Polyrhythm Challenge

- **Nivå:** 3–4. **Læringsmål og teori:** Spille to samtidige pulser med ulik deling.
- **Spillflyt:** Tangent viser separate rytmebaner for hver hånd og gir sakte felles takt.
- **Eksempeloppgave:** Høyre spiller tre jevne anslag mens venstre spiller to i samme tidsrom (3:2).
- **Målbar evaluering / begrensninger:** Vurder begge uavhengige tidsgrid og felles periodestart; ikke krav om håndidentitet uten partmeta.
- **Progresjon:** 3:2 svært sakte → 4:3 → polyrytmisk tekstur med melodier.

### R08 — Groove Lock

- **Nivå:** 1–4. **Læringsmål og teori:** Holde egen puls sammen med trommer og bass uten å bli avhengig av visuell trefflinje.
- **Spillflyt:** Velg backing-groove og spill ett rytmemotiv; systemet viser hvor onset ligger i forhold til grooven.
- **Eksempeloppgave:** Spill popåttendedeler over trommekomp og vurder hvor stabilt offbeat holdes.
- **Målbar evaluering / begrensninger:** Bruk reell backing-clock og kalibrert timing; track gjennomsnittlig avvik og spredning.
- **Progresjon:** Langsom enkel groove → synkope → skiftende swing-følelse og uventede dropp.


## 8. Gehør og musikalsk hukommelse (E01–E08)

Gehørøvelser skal først og fremst utvikle relative relasjoner, ikke kreve medfødt absolutt gehør. Referanselyd må aldri utilsiktet lekke gjennom forventet-tangent-glød eller fasit før svaret.

### E01 — Higher or Lower

- **Nivå:** 0–1. **Læringsmål og teori:** Høre hvilken av to isolerte toner som er høyest/lavest.
- **Spillflyt:** Spill to toner sekvensielt; velg høyere, lavere eller lik.
- **Eksempeloppgave:** Hør C4 så G4; spilleren svarer «høyere».
- **Målbar evaluering / begrensninger:** Vurder klassifisering uavhengig av pianoets tonehøyde; kontroller volum-/registerbias.
- **Progresjon:** Store intervaller → små intervaller → lik tone og omvendt rekkefølge.

### E02 — Find That Note

- **Nivå:** 0–3. **Læringsmål og teori:** Lære å matche hørte toner på pianoet, med eller uten referanse.
- **Spillflyt:** Spill én tone; brukeren finner den ved å sammenligne og høre på nytt.
- **Eksempeloppgave:** Hør A4, finn A4; alternativt finn samme pitchklasse i et annet register.
- **Målbar evaluering / begrensninger:** Ved relativ variant: aksepter pitchklasse; ved eksakt variant: krev oktav.
- **Progresjon:** Kjent starttone → begrenset register → større register, mindre repetisjonsmulighet.

### E03 — Interval Ear

- **Nivå:** 1–4. **Læringsmål og teori:** Gjenkjenne intervallklang fra to etterfølgende eller samtidige toner.
- **Spillflyt:** Spill et intervall, spør om kvalitet/størrelse, og la svar bekreftes ved å spille det.
- **Eksempeloppgave:** C4–E4 = stor ters; C4–E♭4 = liten ters.
- **Målbar evaluering / begrensninger:** Evaluer valgt intervall eller totoners MIDI-respons; varier rot/register for å unngå ren lydbasert memorering.
- **Progresjon:** Oktav/kvint/ters → alle intervaller → omvendinger og harmonisk form.

### E04 — Chord Detective

- **Nivå:** 1–4. **Læringsmål og teori:** Gjenkjenne akkordkvaliteter med øret.
- **Spillflyt:** Spill blind akkord uten visuell fasit; brukeren velger/spiller kvalitet.
- **Eksempeloppgave:** Lytt til C-dur vs Cm vs Cdim vs Caug; senere C7/Cmaj7/Cm7.
- **Målbar evaluering / begrensninger:** Score kvalitet separat fra gjenkjenning av grunntone og voicing.
- **Progresjon:** Dur/moll → dim/aug → septimakkorder i inversjoner.

### E05 — Scale Detective

- **Nivå:** 2–4. **Læringsmål og teori:** Identifisere skalaens karakteristiske intervallmønster ved lytting.
- **Spillflyt:** Spill én skala eller improvisert frase over pedalpunkt; velg modus/skalatypen.
- **Eksempeloppgave:** Hør D-dorisk kontra D-naturlig moll, særlig forskjellen mellom H og B♭.
- **Målbar evaluering / begrensninger:** Vurder riktig skala/mode; varier starttoner for å hindre registermemorisering.
- **Progresjon:** Dur/moll → pentatonisk/blues → modi og melodisk/harmonisk moll.

### E06 — Melody Echo

- **Nivå:** 0–4. **Læringsmål og teori:** Utvikle melodisk hukommelse og evne til å spille etter gehør.
- **Spillflyt:** Avspill to til åtte toner og skjul notebildet; brukeren gjenskaper tonene.
- **Eksempeloppgave:** C4–E4–G4–E4, deretter lengre motiv med ett sprang.
- **Målbar evaluering / begrensninger:** Vurder relativ kontur i nybegynnermodus, eksakt pitch/intervaller i senere nivå og rytme separat.
- **Progresjon:** Tre noter → lengre fraser → transponerte eller polyfone ekko.

### E07 — Rhythm Dictation

- **Nivå:** 1–4. **Læringsmål og teori:** Høre metrisk struktur og notere eller spille korrekt rytme.
- **Spillflyt:** Hør rytme uten synlige noteheads, gjenta på én tangent eller plasser rytmetegn i UI.
- **Eksempeloppgave:** To takter 4/4 med synkopert åttendedelsfigur.
- **Målbar evaluering / begrensninger:** Sammenlign anslag, varigheter og pauser; toleranser følger tempo og kalibrering.
- **Progresjon:** Én takt → flere → 6/8, synkoper, punktering og tuplett.

### E08 — Harmonic Detective

- **Nivå:** 2–4. **Læringsmål og teori:** Høre bassretning, funksjon og akkordprogresjoner.
- **Spillflyt:** Spill kort harmonisk sekvens, og be om romertall, bass eller akkordkvalitet.
- **Eksempeloppgave:** Hør I–V–vi–IV i C, deretter samme funksjoner i G.
- **Målbar evaluering / begrensninger:** Vurder funksjonsrekkefølge separat fra absolutte akkordnavn; tilfeldig transponering hindrer utenatlæring.
- **Progresjon:** To funksjoner → popfireklang → kadens med septimer og modulasjonsindikatorer.


## 9. Akkompagnement, improvisasjon og komposisjon (C01–C09)

Disse øvelsene må skille deterministiske oppgaver (spesifikt komp) fra kreative oppdrag der mange toner, rytmer og uttrykk er gyldige.

### C01 — Bass Fundamentals

- **Nivå:** 0–2. **Læringsmål og teori:** Bygge stabilt venstrehåndskomp med grunntone og kvint.
- **Spillflyt:** Vis akkordsymbol og bassmønster som en egen staff; spill over enkel melodi.
- **Eksempeloppgave:** C-akkord: C3–G3–C3–G3 i kvartpuls.
- **Målbar evaluering / begrensninger:** Vurder riktig grunntone/kvint, tempo og posisjon; hånd krever eksplisitt register/part.
- **Progresjon:** Én akkord → flere akkorder → oktavbass og innpassing i 4/4.

### C02 — Waltz Accompaniment

- **Nivå:** 1–3. **Læringsmål og teori:** Spille tydelig valsetakt: bass på første slag, akkord på to og tre.
- **Spillflyt:** Tangent fremhever bassparti og akkordslag separat mens melodi kan høres som støtte.
- **Eksempeloppgave:** I C-dur 3/4: C3 på 1; C4–E4–G4 samlet på 2 og 3.
- **Målbar evaluering / begrensninger:** Bedøm mønsteret slagvis, akkordsett og tydelig metrum, ikke bare totalt antall treff.
- **Progresjon:** Én akkord → I–IV–V → melodi i høyre hånd og rubato som avansert variant.

### C03 — Alberti Bass

- **Nivå:** 1–3. **Læringsmål og teori:** Lære klassisk brutte-akkordmønster lav–høy–midt–høy.
- **Spillflyt:** Vis akkorden som basis og spesifikk melodisk rekkefølge i bassen.
- **Eksempeloppgave:** C-dur: C3–G3–E3–G3 repetert i åttendedeler.
- **Målbar evaluering / begrensninger:** Sjekk bestemt rekkefølge og jevnt onset; feiltoner skiller fra riktig akkord i feil mønster.
- **Progresjon:** Langsomt over ett harmonisk felt → akkordskifter → to-hånds klassisk frase.

### C04 — Pop Arpeggios

- **Nivå:** 1–3. **Læringsmål og teori:** Bygge typiske repeterende akkompagnementsmønstre.
- **Spillflyt:** Velg akkordloop og figurer som 1–5–3–5 eller 1–5–8–5.
- **Eksempeloppgave:** C–G–E–G gjentas før overgang til Am: A–E–C–E.
- **Målbar evaluering / begrensninger:** Mål mønster mot riktig akkord og onsetgrid; la kreativ variant være fri for eksakt mønsterkrav.
- **Progresjon:** To akkorder → fire akkorder → akkompagner melodier og variere mønster.

### C05 — Four Chord Challenge

- **Nivå:** 1–3. **Læringsmål og teori:** Forstå grunnleggende funksjonell popharmonikk.
- **Spillflyt:** Spill I–V–vi–IV i valgt toneart og hold komp med venstre/høyre hånd.
- **Eksempeloppgave:** C–G–Am–F med fire slag per akkord.
- **Målbar evaluering / begrensninger:** Evaluer akkordidentitet og riktig rekkefølge; omvendinger aksepteres dersom oppgaven er fri.
- **Progresjon:** C-dur → andre tonearter → tidsbestemt akkordbytte og egendefinert voicing.

### C06 — Twelve Bar Blues

- **Nivå:** 2–4. **Læringsmål og teori:** Forstå 12-takters bluesstruktur, I7–IV7–V7 og bassgroove.
- **Spillflyt:** Vis tydelig harmonisk tidslinje og backing; spille akkorder eller walking/shuffle-basstema.
- **Eksempeloppgave:** I C: fire takter C7, to F7, to C7, én G7, én F7, to C7 (én vanlig grunnvariant).
- **Målbar evaluering / begrensninger:** Vurder harmonisk plassering og rekkefølge; alternative bluesvarianter må være eksplisitt valgt.
- **Progresjon:** Akkordrøtter → fulle septimer → shuffle/swing og turnaround.

### C07 — Improvisation Garden

- **Nivå:** 0–4. **Læringsmål og teori:** Utforske egne melodier over en stabil harmonisk ramme.
- **Spillflyt:** Velg skala, register og backing; la valgfri musikalsk respons styre visuelle reaksjoner.
- **Eksempeloppgave:** Improviser med C-mollpentatonisk over en enkel Cm-groove.
- **Målbar evaluering / begrensninger:** Ikke gi falsk rett/feil-score på kreativitet; vis deskriptiv feedback for skala-tilhørighet, variasjon og rytme.
- **Progresjon:** Tre tillatte toner → pentatonikk → blues, modale og akkordtonefokus.

### C08 — Call and Response

- **Nivå:** 1–4. **Læringsmål og teori:** Utvikle musikalsk dialog, motivvariasjon og timing.
- **Spillflyt:** Spill et motiv; brukeren besvarer med repetisjon, kontrast eller avslutning i neste takt.
- **Eksempeloppgave:** Tangent spiller C–D–E, spilleren svarer E–D–C over samme puls.
- **Målbar evaluering / begrensninger:** I eksakt ekko bedøm noter; i fri respons mål turfordeling og valgfri stilistisk likhet uten absolutt fasit.
- **Progresjon:** To takter → lengre setninger → harmoniske spørsmål/svar med modulering.

### C09 — Lead Sheet Player

- **Nivå:** 2–4. **Læringsmål og teori:** Spille fra enkelt melodinotebilde og akkordsymboler, ikke full arrangementnotasjon.
- **Spillflyt:** Vis melodi øverst, besifring over takt, og gi valg mellom akkompagnementsnivåer.
- **Eksempeloppgave:** Melodi i C med akkordsymbolene C | Am | F | G7.
- **Målbar evaluering / begrensninger:** Bedøm notert melodi eksakt, og akkordkomp som korrekt harmonisk sett i riktig takt.
- **Progresjon:** Enkle poplåter → jazzstandard-former med septimer og friere voicinger.


## 10. Musikalitet, innstudering og fremføring (P01–P08)

Denne kategorien gjenbruker Tangents Play/Train, A/B-loop, partvalg og lagrede forsøk der de passer. Nye kriterier for uttrykk må bygges og testes før de vises som objektiv fasit.

### P01 — One-Hand Practice

- **Nivå:** 0–4. **Læringsmål og teori:** Isolere én hånd i et stykke med tydelig høyre-/venstremål.
- **Spillflyt:** Velg eksplisitt notepart eller demonstrert håndmapping; Tangent spiller eventuelt den andre stemmen.
- **Eksempeloppgave:** Øv venstrehåndsostinato alene mens høyremelodi spilles som referanse.
- **Målbar evaluering / begrensninger:** Score bare valgte noter; ikke finn på håndpartisjoner for flerstemmige MIDI-filer uten metadata.
- **Progresjon:** Én hånd sakte → bytte hånd → begge med gradvis svakere referanse.

### P02 — Measure Loop

- **Nivå:** 0–4. **Læringsmål og teori:** Automatisere vanskelige passasjer gjennom korte og målbare repetisjoner.
- **Spillflyt:** Sett A/B rundt problemområde; gjenta segmentet og vis progresjon mellom sammenlignbare forsøk.
- **Eksempeloppgave:** Loop takt 9–12 i MusicXML med 70 % tempo og valgt part.
- **Målbar evaluering / begrensninger:** Bruk faktisk note-hit-coverage og forsøkets kontekst; ikke kall dagens coverage-score timingkvalitet.
- **Progresjon:** Fra to takter → hele frase; søk automatisk svake seksjoner bare når hitdata finnes.

### P03 — Tempo Ladder

- **Nivå:** 0–4. **Læringsmål og teori:** Øke flytende tempo kontrollert uten at feil blir permanent innøvd.
- **Spillflyt:** Start i avtalt hastighet; gå ett lite trinn opp etter flere stabile, fullførte repetisjoner.
- **Eksempeloppgave:** Spill åtte takter ved 60 %, deretter 65 %, 70 % når definert mål er møtt.
- **Målbar evaluering / begrensninger:** Før rytmisk evaluator finnes kan bare pitchdekning brukes og dette må merkes; ikke presenter pitch alene som tempo-mestring.
- **Progresjon:** Fra 50–60 % → eget måltempo; stopp automatisk økning ved økt feilrate.

### P04 — Sight-Reading Arena

- **Nivå:** 0–4. **Læringsmål og teori:** Lese hittil ukjente noter i sammenhengende puls uten memorering.
- **Spillflyt:** Generer korte originale fraser på brukerens nivå; ingen forhåndsavspilling i testmodus.
- **Eksempeloppgave:** Åtte takter i G-dur med sekunder/terser og enkle kvart-/åttendedelsrytmer.
- **Målbar evaluering / begrensninger:** Mål førstegangsutførelse; ikke gjenbruk samme materiale som «ukjent» i senere vurdering.
- **Progresjon:** Tre tonehøyder → begge nøkler → flere stemmer, tonearter og rytmer.

### P05 — Dynamic Control

- **Nivå:** 1–4. **Læringsmål og teori:** Spille piano/forte, crescendo og diminuendo bevisst.
- **Spillflyt:** Gi korte gjentatte toner eller en frase med uttrykkskurve; vis anslagsstyrke mot relativ referanse.
- **Eksempeloppgave:** Fire toner gradvis sterkere i crescendo; neste fire svakere i diminuendo.
- **Målbar evaluering / begrensninger:** Vurder velocity-trend og relativt spenn, ikke absolutte lyd-dB; kalibrer instrumentets velocity-respons.
- **Progresjon:** To tydelige styrker → gradvis kurve → uavhengig balanse mellom stemmer.

### P06 — Articulation Challenge

- **Nivå:** 1–4. **Læringsmål og teori:** Fremføre legato, staccato og frasering i kontekst.
- **Spillflyt:** En kort melodi vises med buer og prikker; spilleren velger lydlig uttrykk.
- **Eksempeloppgave:** C–D–E–F spilles først legato, senere staccato i samme tempo.
- **Målbar evaluering / begrensninger:** Mål note-on/off/overlapp med pedalpolicy og frasetoleranse; ikke vurder usynlig håndbevegelse.
- **Progresjon:** En stil om gangen → veksling → lengre fraser og blandet artikulasjon.

### P07 — Pedal Master

- **Nivå:** 2–4. **Læringsmål og teori:** Forstå sustainpedal, pedalskift og harmonisk klarhet.
- **Spillflyt:** Spill akkordbytter med viste pedalmarkeringer; forklar CC64 og instrumentforskjeller.
- **Eksempeloppgave:** Hold C-akkord med pedal, slipp/skift ved overgang til G slik at harmoniene ikke flyter sammen.
- **Målbar evaluering / begrensninger:** Les CC64 når tilgjengelig; ved manglende pedal meld oppgaven ikke vurderbar, ikke «feil».
- **Progresjon:** Enkle pedalskift → synkopert pedal → lengre harmoniske progresjoner.

### P08 — Concert Mode

- **Nivå:** 1–4. **Læringsmål og teori:** Trene helhetlig fremføring uten gjentatt avbrytelse.
- **Spillflyt:** Start et valgt stykke, slå av hints, ingen automatisk omstart; vis evaluering først etter fullført opptreden.
- **Eksempeloppgave:** Spill «First light» fra start til slutt med forventede noter skjult i eksamensmodus.
- **Målbar evaluering / begrensninger:** Bruk bare reelt støttede metrikker; klart skille mellom pitchdekning, timing og uttrykk.
- **Progresjon:** Kort stykke → lengre repertoar, publikums-/prestasjonssimulering og frivillig opptak.
## Pedagogisk produktmodell

### Seks delte øvelsesmotorer, én teorimodul

| Domenetjeneste | Oppgaver | Foreslått ansvar |
|---|---|---|
| `NoteChallenge` | N01–N08, S10, H02 | Generere notebilde/tonemål, vurdere pitch og stavemåte når relevant. |
| `SequenceRunner` | T01–T08, S02–S09, A01–A07, K04, P02–P04 | Forventet sekvens, rekkefølge, note-on/off og progresjon. |
| `ChordMatcher` | H03–H09, K05–K07, C01–C06, C09 | Tonemengde, tidsvindu, omvending og basskrav. |
| `RhythmEngine` | R01–R08, P05–P07 | Beatgrid, varigheter, pauser, latency og pedal/velocity. |
| `ListenAndRespond` | E01–E08, C08 | Stimulusavspilling, skjult fasit, lydrepetisjoner og svar. |
| `CreativeSandbox` | C07–C09, K08, frie moduser | Backing, skala/harmoni og ikke-normativ tilbakemelding. |
| `TheoryChallenge` (delt komponent) | S01, H01–H09, K01–K03, K08 | Intervallsymbol, stavemåte, akkordfunksjon, kvintsirkel og flervalg. |

**Dette er domenenavn som forslag, ikke eksisterende Rust-moduler.** Musikkregler skal være uavhengige av Bevy, browser/MIDI-adapter og SQLite. UI viser øvelsens tilstand og input. Følg arkitekturprinsippene i [AGENTS.md](../AGENTS.md) og registrer en ADR når konkrete modell-, timing- eller lagringskontrakter faktisk bestemmes.

### Felles øvelseskontrakt (utkast, ikke fast JSON-schema)

```yaml
id: S03
version: 1
title: Major Scale Run
category: scales
engine: sequence
level: 1
concepts: [major-scale, key-signature, fingering]
parameters:
  key: D-major
  octaves: 1
  hands: right
  bpm: 60
  direction: up-down
  notation: staff
  hints: optional
scoring:
  pitch: exact
  sequence: required
  onset_timing: only-if-calibrated
  fingering: advisory-only
progression:
  next: [two-octaves, left-hand, no-hints]
```

Én instans definerer `exercise_id`, versjon, parametere, forventet stimulus/noter, tillatte svar/voicinger, evalueringsmetrikker og tilgjengelige hints. Detaljerte forsøksdata må knyttes til den **eksakte genererte instansen**, ikke bare ID-en, slik at en tilfeldig oppgave kan reproduseres via seed.

### Spillsløyfe og feedback

1. **Velg mål** etter ferdighetsgraf, ønsket øktlengde og registrert svakhet; foreslått snarere enn tvungent.
2. **Forbered** med kort forklaring, hørbar demonstrasjon og skala-/akkordillustrasjon når det er pedagogisk relevant.
3. **Spill** med MIDI eller on-screen/keyboard fallback. Unngå å blokkere keyboard hvis Web MIDI mangler.
4. **Vurder** bare målbare dimensjoner for øvelsen, med toleranser og hjelpstatus synlige.
5. **Forklar feil**: for høy/lav pitch, feil intervall, manglende akkordtone, sen/tidlig onset, pedal eller teori – aldri bare rød blink.
6. **Øv igjen** i justert tempo/register eller med ett hint mindre. Tilby pause og frivillig slutt.
7. **Lagre** forsøk og eksplisitt mestringsnivå, ikke «fremgang» utledet fra ett tilfeldig forsøk.

Gjenbruk gjeldende prinsipp om *solid fill = fysisk spilt tone* og *edge glow = forventet/avslørt tone* fra [UI/UX v2](design/tangent-ui-v2.md). Støtt redusert bevegelse, tydelig tekst ved feil og farger fra brukerens tema. Automatisk referanseavspilling må aldri gi poeng som om eleven spilte.

### Separate mestringsdimensjoner

- **Pitch accuracy:** riktige, manglende og ekstra pitch-eventer.
- **Timing:** anslagsavvik, spredning og tempo relativt til kalibrert tidsakse.
- **Duration/articulation:** note-off, legato-overlapp/gap, varigheter og pedalstatus når tilgjengelig.
- **Consistency:** variasjon gjennom repetisjoner, ikke bare beste forsøk.
- **Theory:** riktig teoretisk stavemåte, akkordsymbol, funksjon, intervall, rytmeverdi.
- **Listening:** identifisere/gjenskape hørte forhold; skille notehukommelse fra absolutt gehør.
- **Expression:** relativ velocity/dynamisk kurve der MIDI støtter målingen; aldri antatt lydnivå ved ekstern piano-output.
- **Creative:** beskrivende metrikker, aldri universell kvalitetsdom for improvisasjon.

**Statuskrav:** Innfør en ny score-type først når dens faktiske beregning og begrensninger er dokumentert. Historiske pitch-dekningstall skal ikke vises som timing- eller mastery-score.

### Trinnvis utrulling

| Bølge | Foreslått førsteinnhold | Behov |
|---|---|---|
| A – grunnmotorer | N01, N03, T01, S02, S03, H01, H03, A01, R01, E01 | Eksisterende Train-teknikk, bedre parametergenerering og avgrenset inputevaluering. |
| B – harmonisk teori | S04, H04–H07, K01–K07, A02, E03–E04 | Stavemåte-/toneartsdomene, akkordmatcher og kvintsirkel-visning. |
| C – rytme og gehør | R02–R08, E02, E05–E08, P05–P07 | Kalibrert klokke, sekvensavspilling, MIDI note-off/velocity/CC64, ærlig feilhåndtering. |
| D – kreativitet og viderekommen | S07–S09, H08–H09, K08, C01–C09, P04/P08 | Backing, nye noter, fleksible voicinger, ikke-normativ feedback og repertoar-kobling. |

Rull ut én representativ øvelse per domenemotor før mange variasjoner. Merk hver øvelse separat som `proposed`, `in-progress`, `implemented`, `validated-with-MIDI` i produktdata; statusene i denne katalogen er **forslag**. For MIDI-validering: test på ekte Yamaha P-45 og simulerte input-events, ulike oktaver og utsatte note-off/release; for web og native må begge veier valideres.

### Daily Practice — eksempel på 20-minutters session (ikke levert)

| Tid | Øvelse | Ferdighet |
|---|---|---|
| 3 min | N01 Note Hunter | Notelesing |
| 4 min | S03 Major Scale Run | Skala og fingermønster |
| 4 min | H03 Chord Constructor | Harmonisk grunnforståelse |
| 3 min | R04 Rhythm Echo | Timing og rytme |
| 3 min | E03 Interval Ear | Relativt gehør |
| 3 min | C07 Improvisation Garden | Kreativ anvendelse |

Øktbygger skal fordele ferdighetene, følge brukerens eksplisitte mål og foreslå repetisjon av svake områder. Ingen tap-av-streak-straff; fridager og belastningsgrenser må respekteres.

## Verifikasjonskriterier for fremtidig implementasjon

1. Hver ID kan generere minst tre nivåtilpassede varianter, og generering er deterministisk for samme seed/parametere.
2. For hver øvelse finnes minst ett «riktig», ett «feil» og ett grensetilfelle; domenetester dekker disse.
3. ChordMatcher skiller mellom uordnet pitchklasse-sett, registrert bass og faktisk tidsvindu.
4. Teorimotor skiller enharmonisk lik MIDI-pitch fra riktig stavet svar; norsk/internasjonal navneprofil testes.
5. Latency/tempo og uassistert/assistert tilstand lagres som eksplisitt kontekst for sammenlignbar score.
6. Hånd-/fingerkrav oppgis uten å påstå fysisk identifikasjon som MIDI ikke leverer.
7. Manglende MIDI/pedal/velocity gir redusert, tydelig merket vurdering, ikke oppdiktet score.
8. Hints og referanselyd påvirker scoring eksplisitt; frivillig repetisjon av stimulus logges.
9. Ingen øvelse kan slette/migrere eksisterende musikkbibliotek, score eller preferanser uten kompatibilitetstest.
10. Alle øvelser kan avbrytes/pause og tilpasses begrenset skjerm, fargeblindhet og redusert bevegelse.

## Faglige og produktmessige referanser

- [ABRSM – Piano](https://www.abrsm.org/en-gb/instruments/piano): teknikk, scales, sight-reading og repertoar.
- [RCM – Curriculum and Disciplines](https://www.rcmusic.com/learning/about-the-royal-conservatory-certificate-program/curriculum-and-disciplines): repertory, technique, ear training og theory.
- [Trinity College London – Piano technical work](https://www.trinitycollege.com/qualifications/music/grade-exams/piano/resources/piano-technical-work): skala-/arpeggioarbeid.
- [musictheory.net – Exercises](https://www.musictheory.net/exercises): eksempler på interaktive teorikategorier.
- [Music Theory for the 21st-Century Classroom](https://musictheory.pugetsound.edu/mt21c/): åpne definisjoner av intervaller, tonearter, harmoni og funksjon.
- [Piano Marvel](https://pianomarvel.com/): produktreferanse for prima vista/progresjon; **ikke kilde til kopiering av kode, øvingsfiler eller UI**.
- [Melodics](https://melodics.com/): produktreferanse for timing-feedback.
- [Yousician](https://yousician.com/): produktreferanse for guidede økter.

Disse kildene er faglige referanser/benchmark, ikke en påstand om at Tangent er godkjent av dem. For kommersielt undervisningsinnhold gjelder egne rettighetsvurderinger; generer originale øvelser og unngå kopi av beskyttede arrangementer, noter, grafikk eller lyd.
