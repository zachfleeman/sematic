CREATE TABLE verb_frame (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  frame TEXT NOT NULL,
  members TEXT[] NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  created_by UUID NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_by UUID NOT NULL
);

SELECT sqlx_manage_updated_at('verb_frame');