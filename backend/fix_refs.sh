#!/bin/bash
# Quick fix script to remove all references to deleted types

# 1. Remove from repository/mod.rs
sed -i '' '/ContentModificationRepository/d' src/infrastructure/repository/mod.rs
sed -i '' '/SynthesisRepository/d' src/infrastructure/repository/mod.rs

# 2. Remove from repository/course/mod.rs  
sed -i '' '/ContentModificationRepository/d' src/infrastructure/repository/course/mod.rs
sed -i '' '/SynthesisRepository/d' src/infrastructure/repository/course/mod.rs

# 3. Remove from infrastructure/mod.rs
sed -i '' '/ContentModificationRepository/d' src/infrastructure/mod.rs
sed -i '' '/SynthesisRepository/d' src/infrastructure/mod.rs
sed -i '' '/SupabaseContentModificationRepository/d' src/infrastructure/mod.rs
sed -i '' '/SupabaseSynthesisRepository/d' src/infrastructure/mod.rs

# 4. Remove from supabase/mod.rs
sed -i '' '/SupabaseContentModificationRepository/d' src/infrastructure/supabase/mod.rs
sed -i '' '/SupabaseSynthesisRepository/d' src/infrastructure/supabase/mod.rs

# 5. Remove from supabase/course/mod.rs
sed -i '' '/SupabaseContentModificationRepository/d' src/infrastructure/supabase/course/mod.rs
sed -i '' '/SupabaseSynthesisRepository/d' src/infrastructure/supabase/course/mod.rs

# 6. Remove from tests/intello/helpers.rs
sed -i '' '/ContentModificationRepository/d' src/tests/intello/helpers.rs
sed -i '' '/SynthesisRepository/d' src/tests/intello/helpers.rs
sed -i '' '/StubContentModificationRepository/d' src/tests/intello/helpers.rs
sed -i '' '/StubSynthesisRepository/d' src/tests/intello/helpers.rs

# 7. Remove from tests/intello/property_tests.rs
sed -i '' '/StubContentModificationRepository/d' src/tests/intello/property_tests.rs
sed -i '' '/StubSynthesisRepository/d' src/tests/intello/property_tests.rs

# 8. Remove from app.rs
sed -i '' '/SupabaseContentModificationRepository/d' src/app.rs
sed -i '' '/SupabaseSynthesisRepository/d' src/app.rs
sed -i '' '/content_modification_repo/d' src/app.rs
sed -i '' '/synthesis_repo/d' src/app.rs

# 9. Remove from services/intello/types.rs  
sed -i '' '/ContentModificationRepository/d' src/services/intello/types.rs
sed -i '' '/SynthesisRepository/d' src/services/intello/types.rs
sed -i '' '/content_modification_repo/d' src/services/intello/types.rs
sed -i '' '/synthesis_repo/d' src/services/intello/types.rs

echo "Imports removed - now manually delete the repository implementations"
