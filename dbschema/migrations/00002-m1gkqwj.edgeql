CREATE MIGRATION m1gkqwjv3d2cmzdw2l2432ckiflcglfkw4eb2ssqaaq7qagdfztu7q
    ONTO m1mxvnntu7bolxeccw3dvadj5w33fm4gllltqal5mhyaiwqhgbkpoq
{
  CREATE TYPE default::Session EXTENDING default::Node {
      CREATE REQUIRED LINK user: default::User;
      CREATE REQUIRED PROPERTY expires_at: std::datetime;
  };
  ALTER TYPE default::User {
      CREATE LINK sessions := (.<user[IS default::Session]);
  };
};
