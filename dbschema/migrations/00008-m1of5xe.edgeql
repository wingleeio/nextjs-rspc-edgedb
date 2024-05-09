CREATE MIGRATION m1of5xeyno6pthqujfxhkmfkts2dgza7p6h2u6byix6fnirdjssvfa
    ONTO m1njye5c4pfbsdvevqojh4h6copaugbomt3opr3lmqhbyf7stuusha
{
  ALTER TYPE default::Session {
      CREATE PROPERTY browser_name: std::str;
      CREATE PROPERTY browser_version: std::str;
      CREATE PROPERTY os_name: std::str;
      CREATE PROPERTY os_version: std::str;
  };
};
