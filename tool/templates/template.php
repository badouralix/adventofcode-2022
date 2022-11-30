<?php

function run(string $input): int
{
    // your code here
    return 0;
}

$startTime = microtime(true);
$answer = run($argv[1]);
$endTime = microtime(true);

fwrite(STDOUT, sprintf("_duration:%f\n", ($endTime - $startTime)*1000));
fwrite(STDOUT, $answer."\n");
