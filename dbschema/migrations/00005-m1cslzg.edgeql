CREATE MIGRATION m1cslzg5o2jwwjvaojfjupufe5k6g5u73umxfuvjwv5ynmlno2kb3q
    ONTO m1ci6nse5yy2vrto5ll6am3szsxn2uc57nu7nzub7c7cxllfiifcwa
{
  ALTER TYPE default::User {
      ALTER PROPERTY email {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE INDEX ON (.email);
  };
};
