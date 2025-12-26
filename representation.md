# About the Repesentation
Accidentals applied are a complicated system of displacement from their diatonic
representation. Even more complicated is the representation of interval as some intervals like
unison have perfect versions but no minor or major versions and some have minor and minor
versions but no perfect version.
This library solves this problem by representing pitches and intervals by diatonic steps and
chromatic steps.
Diatonic steps are equivalent to staff positions. So the number of diatonic steps of an interval
which shifts a pitch up by one staff position (e.g. C to D) is 1.
Note that this is always one less than the usual name of an interval. For example a third is
represented by 2 diatonic steps. This shift results from music theory using one indexing while
this crate uses zero indexing for ease of implementation of operations like addition and
negation.

Chromatic steps are more self explanatory. They just counts the chromatic steps which are taken
from one note to another. So the chromatic steps of the interval from C to D are 2.

Pitches can now be expressed as intervals to middle c (C4 in scientific pitch notation).
This makes the transpostition of notes very simple. When the name of a note is needed the pitch
name and the accidental can then be calculated from this representation.
This way all transpositions can be represented correctly, but allow for arbitrary number of
flats and sharps on any note.

To reduce the complexity for the user of this crate FromStr implementations for all types which
face this problem are implemented using representations familliar to a user knowing music
theory.

A side effect of this way to implement musical objects is that it allows the representation of
object you will never encouter in practice. For example there's nothing preventing a user from
creating a pitch like quadruply sharp B.
Wherever this was encountered some self constistent system was tried to be followed, but proper
display and parsing of such objects is not the focus of this crate.
