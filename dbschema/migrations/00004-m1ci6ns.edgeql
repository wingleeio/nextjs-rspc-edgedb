CREATE MIGRATION m1ci6nse5yy2vrto5ll6am3szsxn2uc57nu7nzub7c7cxllfiifcwa
    ONTO m1w4keia4xr74sn34zksutjfkkjlbds3buy7hj7mgqawivtdxhv7ha
{
  ALTER TYPE default::Node {
      CREATE PROPERTY id_str := (<std::str>.id);
  };
};
