<html>

  <head>
    <title>Quote Testing in PHP</title>
  </head>
  <body>
    <h1> Quote Testing </h1>
    <?php
      $test1 = "String with \"Escaped quotes\"!";
      $test2 = 'String with "Single quotes"!';
      $test3 = "String with 'Single and double quotes'.";

      echo "<p>$test1</p>";
      echo "<p>$test2</p>";
      echo "<p>$test3</p>";
    ?>
    
  </body>
</html>