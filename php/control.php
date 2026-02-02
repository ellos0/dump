<html>

<head>
  <title>Control Flow in PHP</title>
</head>
<body>
  <?php
    $i = 0;
    $fam = ["Babylon", "Katie", "Dvorak"];
    while ($i < sizeof($fam)) {
          switch ($fam[$i][0]) {
                 case 'K':
                      echo "Person whose name starts with K<br>\n";
                      break;
                 case 'B':
                      echo "Person whose name starts with B<br>\n";
                      break;
                 default:
                      echo "Other name: $fam[$i]";
          }
          $i+= 1;
    }
  ?>
</body>
</html>