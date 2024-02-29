# FragCalc

This is a simple program that will calculate the fragments used and the ones still needed based off levels of your various 6th job skills in MapleStory. This is a command line tool and needs to be run using a Power Shell or the Command Line on windows.

Example table output:
```
+--------------------+-------------+------------------+----------------+-----------------+
| Name               | Type        | Fragments Needed | Fragments Used | Percentage Done |
+--------------------+-------------+------------------+----------------+-----------------+
| Spirit Calibur     | Skill       | 4400             | 1660           | 37.73%          |
+--------------------+-------------+------------------+----------------+-----------------+
| HEXA Raging Blow   | Mastery     | 2252             | 2252           | 100.00%         |
+--------------------+-------------+------------------+----------------+-----------------+
| Burning Soul Blade | Enhancement | 3383             | 1092           | 32.28%          |
+--------------------+-------------+------------------+----------------+-----------------+
| Instinctual Combo  | Enhancement | 3383             | 1588           | 46.94%          |
+--------------------+-------------+------------------+----------------+-----------------+
| Sword Illusion     | Enhancement | 3383             | 513            | 15.16%          |
+--------------------+-------------+------------------+----------------+-----------------+
```

Users can build themselves using `cargo build --release` or simply download the attached `zip` file in the release section.

Remember to edit the skills.txt with your skill names. Such as the example:
```
Spirit Calibur:Skill
HEXA Raging Blow:Mastery
Burning Soul Blade:Enhancement
Instinctual Combo:Enhancement
Sword Illusion:Enhancement
```
The portion after the `:` can be `Skill`, `Mastery` or `Enhancement`.