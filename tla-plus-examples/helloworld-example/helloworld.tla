-------------------------------- MODULE helloworld --------------------------------

VARIABLE computingstates

TypeOK == computingstates \in {"computing", "idle", "bufferring"}

Init == computingstates = "idle"

StartBuffering == 
    /\ computingstates = "idle"
    /\ computingstates' = "bufferring"

StartProcessing == 
    /\ computingstates = "bufferring"
    /\ computingstates' = "computing"

EndProcessing ==
    /\ computingstates = "computing"
    /\ computingstates' = "idle"

Next == StartBuffering \/ StartProcessing \/ EndProcessing

=============================================================================
