
BEGIN;

-- ============================================================
-- ENUMS
-- ============================================================
CREATE TYPE user_role_type AS ENUM ('user', 'seller', 'moderator', 'admin');
CREATE TYPE user_status_type AS ENUM ('unverified', 'active', 'banned');
CREATE TYPE auth_provider_type AS ENUM ('google');
-- Thêm provider khác sau này (nếu có): ALTER TYPE auth_provider_type ADD VALUE 'apple';

CREATE TYPE order_status_type AS ENUM (
  'pending', 'confirmed', 'processing', 'shipped', 'delivered', 'cancelled', 'returned'
);
CREATE TYPE payment_status_type AS ENUM ('pending', 'paid', 'failed', 'refunded');
CREATE TYPE product_status_type AS ENUM ('draft', 'active', 'archived');

-- ============================================================
-- FUNCTION: tự động cập nhật updated_at, gắn trigger ở cuối file
-- ============================================================
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================
-- TABLE: Categories - Recursive
-- ============================================================
CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE, -- sample: quan-xi =]]
  parent_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT chk_categories_not_self_parent CHECK (parent_id IS DISTINCT FROM id)
);
CREATE INDEX idx_categories_parent_id ON categories(parent_id);

-- ============================================================
-- TABLE: Products
-- ============================================================
CREATE TABLE products (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE RESTRICT, -- không cho xóa category còn sản phẩm
  name TEXT NOT NULL,
  slug TEXT NOT NULL UNIQUE,
  description TEXT,
  base_price DECIMAL(12,2) NOT NULL CHECK (base_price >= 0),
  status product_status_type NOT NULL DEFAULT 'draft',
  deleted_at TIMESTAMPTZ, -- soft-delete: ẩn khỏi catalog thay vì xóa cứng
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_products_category ON products(category_id);
-- slug đã có index từ UNIQUE constraint, không cần tạo thêm
CREATE INDEX idx_products_catalog_browse ON products(category_id, status) WHERE deleted_at IS NULL;

-- ============================================================
-- TABLE: Product Variants
-- ============================================================
CREATE TABLE product_variants (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  sku TEXT NOT NULL UNIQUE, -- unique stock keeping unit (example: TS-WHITE-L)
  price_override DECIMAL(12,2) CHECK (price_override IS NULL OR price_override >= 0),
  stock_quantity INTEGER NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
  deleted_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_variants_product_id ON product_variants(product_id);

-- ============================================================
-- TABLE: Product Images
-- ============================================================
CREATE TABLE product_images (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
  image_url TEXT NOT NULL,
  is_main BOOLEAN NOT NULL DEFAULT FALSE,
  sort_order INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_images_product_id ON product_images(product_id);
CREATE INDEX idx_images_variant_id ON product_images(variant_id);
-- Chỉ 1 ảnh is_main = true cho mỗi sản phẩm, và 1 cho mỗi variant
CREATE UNIQUE INDEX idx_one_main_image_per_product
  ON product_images(product_id) WHERE is_main = true AND variant_id IS NULL;
CREATE UNIQUE INDEX idx_one_main_image_per_variant
  ON product_images(variant_id) WHERE is_main = true AND variant_id IS NOT NULL;

-- ============================================================
-- TABLE: Users (Google-only auth)
-- ============================================================
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email TEXT NOT NULL,
  name TEXT NOT NULL,
  avatar_url TEXT, -- null thì FE tự dùng ảnh mặc định
  description TEXT,
  role user_role_type NOT NULL DEFAULT 'user',
  status user_status_type NOT NULL DEFAULT 'active', -- Google đã verify email nên khỏi cần 'unverified' mặc định
  provider auth_provider_type NOT NULL DEFAULT 'google',
  provider_id TEXT NOT NULL, -- Google ID, là danh tính đăng nhập chính
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  CONSTRAINT uq_users_provider_identity UNIQUE (provider, provider_id)
);
-- Case-insensitive: 'A@b.com' và 'a@b.com' phải là cùng 1 tài khoản
CREATE UNIQUE INDEX idx_users_email_lower ON users (lower(email));

-- ============================================================
-- TABLE: User Sessions
-- ============================================================
CREATE TABLE user_sessions (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  -- Family ID nhóm các token của cùng một thiết bị/lần đăng nhập.
  -- Lần đầu login dùng default, các lần xoay (rotate) token sau truyền lại ID này.
  session_family_id UUID NOT NULL DEFAULT gen_random_uuid(),

  refresh_token_hash TEXT NOT NULL, -- hash (vd SHA-256), không lưu token gốc
  user_agent TEXT, -- Example: Chrome or Brave On Windows

  -- Phục vụ Token Reuse Detection
  is_used BOOLEAN NOT NULL DEFAULT FALSE,

  revoked_at TIMESTAMPTZ, -- hỗ trợ logout / force-logout hoặc bị thu hồi khi phát hiện reuse
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_sessions_family_id ON user_sessions(session_family_id);
CREATE UNIQUE INDEX idx_sessions_refresh_token_hash ON user_sessions(refresh_token_hash);

-- ============================================================
-- TABLE: User Addresses
-- ============================================================
CREATE TABLE user_addresses (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  recipient_name TEXT NOT NULL,
  recipient_phone TEXT NOT NULL,
  address_line TEXT NOT NULL, -- Số nhà, tên đường
  ward TEXT,                  -- Phường/Xã
  district TEXT NOT NULL,     -- Quận/Huyện
  city TEXT NOT NULL,         -- Tỉnh/Thành Phố
  is_default BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_addresses_user_id ON user_addresses(user_id);
-- Chỉ 1 địa chỉ mặc định cho mỗi user
CREATE UNIQUE INDEX idx_one_default_address_per_user
  ON user_addresses(user_id) WHERE is_default = true;

-- ============================================================
-- TABLE: Orders
-- ============================================================
CREATE TABLE orders (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES users(id) ON DELETE SET NULL, -- giữ đơn hàng dù user bị xóa
  address_id UUID REFERENCES user_addresses(id) ON DELETE SET NULL,
  order_number SERIAL UNIQUE, -- khóa nội bộ, tuần tự
  order_code TEXT NOT NULL UNIQUE
    DEFAULT ('ORD-' || to_char(now(), 'YYYY') || '-' || upper(substr(md5(random()::text), 1, 6))),
    -- mã hiển thị cho khách, không tuần tự -> không lộ tổng số đơn đã bán
  status order_status_type NOT NULL DEFAULT 'pending',
  payment_status payment_status_type NOT NULL DEFAULT 'pending',
  shipping_address_snapshot JSONB,
  total_amount DECIMAL(12,2) NOT NULL CHECK (total_amount >= 0),
  shipping_fee DECIMAL(12,2) NOT NULL DEFAULT 0 CHECK (shipping_fee >= 0),
  notes TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_address_id ON orders(address_id);

-- ============================================================
-- TABLE: Order Items
-- ============================================================
CREATE TABLE order_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
  variant_id UUID REFERENCES product_variants(id) ON DELETE SET NULL,
  product_name_snapshot TEXT NOT NULL, -- giữ tên dù sản phẩm/variant gốc bị xóa
  variant_sku_snapshot TEXT NOT NULL,
  quantity INTEGER NOT NULL CHECK (quantity > 0),
  price_at_purchase DECIMAL(12,2) NOT NULL CHECK (price_at_purchase >= 0), -- Snapshot: Price when buying.
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_variant_id ON order_items(variant_id);

-- ============================================================
-- TABLE: Cart
-- ============================================================
CREATE TABLE cart_items (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  variant_id UUID NOT NULL REFERENCES product_variants(id) ON DELETE CASCADE,
  quantity INTEGER NOT NULL CHECK (quantity > 0),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  -- Quan trọng: 1 user không sở hữu quá 1 dòng cho cùng 1 variant,
  -- thêm vào giỏ thì tăng quantity chứ không tách dòng mới
  UNIQUE(user_id, variant_id)
);
-- user_id đã được index gián tiếp qua cột đầu của UNIQUE(user_id, variant_id) ở trên
CREATE INDEX idx_cart_items_variant_id ON cart_items(variant_id);

-- ============================================================
-- TRIGGERS: auto-update updated_at
-- ============================================================
CREATE TRIGGER trg_categories_updated_at BEFORE UPDATE ON categories
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER trg_products_updated_at BEFORE UPDATE ON products
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER trg_variants_updated_at BEFORE UPDATE ON product_variants
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER trg_users_updated_at BEFORE UPDATE ON users
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER trg_addresses_updated_at BEFORE UPDATE ON user_addresses
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER trg_orders_updated_at BEFORE UPDATE ON orders
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
CREATE TRIGGER trg_cart_items_updated_at BEFORE UPDATE ON cart_items
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

COMMIT;
