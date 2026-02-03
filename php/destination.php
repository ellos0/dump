<!DOCTYPE html>

<html>
<head>
  <title>Destination Page</title>
</head>
<body>
  <?php
    $magic_number = $_GET['magic'];
    echo "<h2>Got magic number from previous page: $magic_number</h2>";
  ?>
</body>
</html>