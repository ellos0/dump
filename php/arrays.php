<!DOCTYPE html>

<html>
  <head>
    <title>Array Testing in PHP</title>
  </head>
  <body>
    <div> Data following: </div>
    <?php
      $data = [10,20,30,86];
      for ($i = 0; $i < sizeof($data); $i++) {
          echo "<div>Data $i: $data[$i]</div>";
      }
    ?>