<!DOCTYPE html>

<html>
<head>
  <title>Fibonacci Calculation in PHP</title>
</head>
<body>
  <?php
    function fibonacci($n) {
             if ($n == 0) {
                return 0;
             } elseif ($n == 1) {
                return 1;
             } else {
                return fibonacci($n-1) + fibonacci($n-2);
             }
    }
    $toTest = [0,1,2,3,4,8,10,24];
    for ($i = 0; $i<sizeof($toTest);$i++) {
        echo "Fibonacci number " . $toTest[$i] . " is equal to: " . fibonacci($toTest[$i]) . "<br>\n";
    }
  ?>
</body
</html>