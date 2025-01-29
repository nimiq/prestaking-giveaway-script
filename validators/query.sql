-- Database schema
-- SQL: https://github.com/nimiq/validators-api/blob/fa594a6a51b30b1acdd66b36f0f326d82ef5a528/server/database/migrations/0000_cultured_wallflower.sql
-- Drizzle schema: https://github.com/nimiq/validators-api/blob/fa594a6a51b30b1acdd66b36f0f326d82ef5a528/server/database/schema.ts

SELECT
  v.name AS "Name",
  v.payout_type AS "Payout",
  v.address AS "Address",
  GROUP_CONCAT(
    CASE
      WHEN a.missed > -1 THEN a.epoch_number
      ELSE NULL
    END
  ) AS "Epochs selected",
  COUNT(
    CASE
      WHEN a.missed > -1 THEN 1
      ELSE NULL
    END
  ) AS "Amount of epochs selected",
  GROUP_CONCAT(
    CASE
      WHEN  a.missed  > 0 THEN a.epoch_number
      ELSE NULL
    END
  ) AS "Missed epochs",
  COUNT(
    CASE
      WHEN  a.missed > 0 THEN 1
      ELSE NULL
    END
  ) AS "Amount of total missed epochs"
FROM
  validators v
  LEFT JOIN activity a ON v.id = a.validator_id
  AND a.epoch_number > 0
  AND a.epoch_number <= 100
GROUP BY
  v.address
ORDER BY
  "Amount of total missed epochs" ASC;